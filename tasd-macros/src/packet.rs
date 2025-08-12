use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Expr, ExprArray, ExprLit, Lit, Meta, Type};

macro_rules! parse_type {
    ($($tt:tt)*) => {{
        let ty: syn::Type = syn::parse_quote! { $($tt)* };
        ty
    }}
}

pub(super) fn derive_packet_inner(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let encode_impl = derive_encode(&input);
    let decode_impl = derive_decode(&input);
    
    quote! {
        #encode_impl
        #decode_impl
    }.into()
}

fn derive_encode(input: &DeriveInput) -> proc_macro2::TokenStream {
    let key = parse_key(&input);
    
    let target_name = &input.ident;
    
    match &input.data {
        Data::Struct(s) => {
            let mut encode_fields = Vec::with_capacity(s.fields.len());
            for field in &s.fields {
                if let Some(ident) = field.ident.as_ref() {
                    let first_attr = field.attrs.first().and_then(|attr| attr.path().require_ident().ok());
                    
                    let tokens = match first_attr {
                        Some(attr) if attr == "u8_enum" => quote! {
                            (self.#ident as u8).encode(&mut writer)?;
                        },
                        Some(attr) if attr == "u16_enum" => quote! {
                            (self.#ident as u16).encode(&mut writer)?;
                        },
                        Some(attr) if attr == "u8_string" => quote! {
                            // code ripped from unstable str::floor_char_boundary on 2025-04-18
                            let index = if 255 >= self.#ident.len() {
                                self.#ident.len()
                            } else {
                                let lower_bound = 255 - 3;
                                let new_index = self.#ident.as_bytes()[lower_bound..=255]
                                    .iter()
                                    .rposition(|b| (*b as i8) >= -0x40);
                                
                                // SAFETY: we know that the character boundary will be within four bytes
                                unsafe { lower_bound + new_index.unwrap_unchecked() }
                            };
                            let data = &self.#ident[..index];
                            
                            (data.len() as u8).encode(&mut writer)?;
                            data.encode(&mut writer)?;
                        },
                        _ => quote! {
                            self.#ident.encode(&mut writer)?;
                        }
                    };
                    encode_fields.push(tokens);
                }
            }
            
            let output = quote! {
                impl Encode for #target_name {
                    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
                        let payload = {
                            let mut writer = vec![];
                            
                            #( #encode_fields )*
                            
                            writer
                        };
                        
                        let mut written = 0usize;
                        written += #key.as_slice().encode(writer)?;
                        written += PLen(payload.len()).encode(writer)?;
                        written += payload.encode(writer)?;
                        
                        Ok(written)
                    }
                }
            };
            
            //eprintln!("STRUCT: {output}");
            output
        },
        Data::Enum(_e) => {
            let rep = parse_repr(input);
            
            let output = quote! {
                impl Encode for #target_name {
                    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
                        let payload = {
                            let mut writer = vec![];
                            
                            (*self as #rep).encode(&mut writer)?;
                            
                            writer
                        };
                        
                        let mut written = 0usize;
                        written += #key.as_slice().encode(writer)?;
                        written += PLen(payload.len()).encode(writer)?;
                        written += payload.encode(writer)?;
                        
                        Ok(written)
                    }
                }
            };
            
            //eprintln!("ENUM: {output}");
            output
        },
        _ => panic!("derive packet doesn't support unions")
    }
}


fn derive_decode(input: &DeriveInput) -> proc_macro2::TokenStream {
    let key = parse_key(&input);
    
    let target_name = &input.ident;
    
    match &input.data {
        Data::Struct(s) => {
            let mut decode_fields = Vec::with_capacity(s.fields.len());
            for (i, field) in s.fields.iter().enumerate() {
                let is_last = i + 1 == s.fields.len();
                if let Some(ident) = field.ident.as_ref() {
                    let ty = &field.ty;
                    let first_attr = field.attrs.first().and_then(|attr| attr.path().require_ident().ok());
                    
                    let tokens = match first_attr {
                        Some(attr) if attr == "u8_enum" => quote! {
                            #ident: <#ty>::try_from(u8::decode(reader)?)?,
                        },
                        Some(attr) if attr == "u16_enum" => quote! {
                            #ident: <#ty>::try_from(u16::decode(reader)?)?,
                        },
                        Some(attr) if attr == "u8_string" => quote! {
                            #ident: U8String::decode(reader)?.0,
                        },
                        _ if ty == &parse_type!{ Vec<u8> } && is_last => quote! {
                            #ident: {
                                let offset = reader.stream_position()? - payload_start;
                                let mut buf = vec![0u8; plen - (offset as usize)];
                                reader.read_exact(&mut buf)?;
                                
                                buf
                            },
                        },
                        _ if ty == &parse_type!{ Vec<u64> } && is_last => quote! {
                            #ident: {
                                let offset = reader.stream_position()? - payload_start;
                                let len = plen - (offset as usize);
                                if len % 8 != 0 {
                                    return Err(DecodeError::WrongLength);
                                }
                                let mut buf = vec![0u8; len];
                                reader.read_exact(&mut buf)?;
                                
                                buf.chunks_exact(8).map(|x| u64::from_be_bytes(x.try_into().expect("should never fail"))).collect()
                            },
                        },
                        _ if ty == &parse_type!{ String } && is_last => quote! {
                            #ident: {
                                let offset = reader.stream_position()? - payload_start;
                                let mut buf = vec![0u8; plen - (offset as usize)];
                                reader.read_exact(&mut buf)?;
                                
                                String::from_utf8(buf)?
                            },
                        },
                        _ if ty == &parse_type!{ Option<Box<Packet>> } && is_last => quote! {
                            #ident: {
                                let offset = reader.stream_position()? - payload_start;
                                let len = plen - (offset as usize);
                                if len == 0 {
                                    None
                                } else {
                                    Some(Box::new(Packet::decode(reader)?))
                                }
                            }
                        },
                        _ => quote! {
                            #ident: <#ty>::decode(reader)?,
                        }
                    };
                    decode_fields.push(tokens);
                }
            }
            
            let output = quote! {
                impl Decode for #target_name {
                    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
                        let packet_start = reader.stream_position()?;
                        
                        fn try_decode<R: Read + Seek>(reader: &mut R) -> Result<#target_name, DecodeError> {
                            let parsed_key = <[u8; 2]>::decode(reader)?;
                            if #key != parsed_key {
                                return Err(DecodeError::WrongKey);
                            }
                            
                            let plen = PLen::decode(reader)?.0;
                            
                            let payload_start = reader.stream_position()?;
                            
                            Ok(#target_name {
                                #( #decode_fields )*
                            })
                        }
                        
                        let result = try_decode(reader);
                        if result.is_err() {
                            reader.seek(std::io::SeekFrom::Start(packet_start))?;
                        }
                        
                        result
                    }
                }
            };
            
            output
        },
        Data::Enum(_e) => {
            let rep = parse_repr(input);
            
            let output = quote! {
                impl Decode for #target_name {
                    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
                        let packet_start = reader.stream_position()?;
                        
                        fn try_decode<R: Read + Seek>(reader: &mut R) -> Result<#target_name, DecodeError> {
                            let parsed_key = <[u8; 2]>::decode(reader)?;
                            if #key != parsed_key {
                                return Err(DecodeError::WrongKey);
                            }
                            
                            let plen = PLen::decode(reader)?.0;
                            
                            let payload_start = reader.stream_position()?;
                            
                            Ok(<#target_name>::try_from(<#rep>::decode(reader)?)?)
                        }
                        
                        let result = try_decode(reader);
                        if result.is_err() {
                            reader.seek(std::io::SeekFrom::Start(packet_start))?;
                        }
                        
                        result
                    }
                }
            };
            
            output
        },
        _ => panic!("cannot derive a union")
    }
}

fn parse_key(input: &DeriveInput) -> ExprArray {
    let Some(key_attr) = input.attrs.iter().find(|attr| attr.path().is_ident("key")) else { panic!("derive requires a `#[key = \"[u8, ...]\"]` attribute") };
    
    parse_key_str(key_attr).expect("key attribute should be the `#[key = \"[u8, ...]\"]` style")
}

fn parse_repr(input: &DeriveInput) -> Type {
    let Some(repr_attr) = input.attrs.iter().find(|attr| attr.path().is_ident("repr")) else { panic!("derive enum requires `#[repr(<primitive>)]` attribute") };
    
    repr_attr.parse_args().expect("valid repr argument")
}

fn parse_key_str(attr: &Attribute) -> Result<ExprArray, syn::Error> {
    if let Meta::NameValue(meta) = &attr.meta {
        if meta.path.is_ident("key") {
            if let Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) = &meta.value {
                return lit.parse();
            }
        }
    }
    
    panic!("expected key attribute `#[key = \"[u8, ...]\"]`");
}