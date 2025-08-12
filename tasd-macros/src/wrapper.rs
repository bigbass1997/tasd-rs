use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

pub(super) fn derive_wrapper_inner(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let encode_impl = derive_encode(&input);
    let decode_impl = derive_decode(&input);
    
    quote! {
        #encode_impl
        #decode_impl
    }.into()
}

fn derive_encode(input: &DeriveInput) -> proc_macro2::TokenStream {
    let target_name = &input.ident;
    
    match &input.data {
        Data::Enum(e) => {
            let encode_variants = for_each_variant(e, &quote! {
                p.encode(writer)
            });
            
            let output = quote! {
                impl Encode for #target_name {
                    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, std::io::Error> {
                        match self {
                            #( #encode_variants )*
                        }
                    }
                }
            };
            
            output
        }
        _ => panic!("derive wrapper only supports enums"),
    }
}

fn derive_decode(input: &DeriveInput) -> proc_macro2::TokenStream {
    let target_name = &input.ident;
    
    match &input.data {
        Data::Enum(e) => {
            let variant_idents = e.variants
                .iter()
                .filter(|v| v.attrs.iter().find(|attr| attr.path().is_ident("unsupported")).is_none())
                .map(|v| &v.ident);
            
            
            let unsupported_ident = e.variants
                .iter()
                .find(|v| v.attrs.iter().find(|attr| attr.path().is_ident("unsupported")).is_some())
                .map(|v| &v.ident)
                .expect("derive wrapper requires a variant tagged with `#[unsupported]`");
            
            let output = quote! {
                impl Decode for #target_name {
                    fn decode<R: Read + Seek>(reader: &mut R) -> Result<Self, DecodeError> {
                        let packet_start = reader.stream_position()?;
                        
                        let mut packet = None;
                        #(
                            packet = <#variant_idents>::decode(reader).map(|inner| inner.into()).ok();
                            if packet.is_some() {
                                return Ok(packet.unwrap());
                            }
                            reader.seek(std::io::SeekFrom::Start(packet_start))?;
                        )*
                        
                        if packet.is_none() {
                            let result = <#unsupported_ident>::decode(reader);
                            match result {
                                Err(err) => {
                                    reader.seek(std::io::SeekFrom::Start(packet_start))?;
                                    return Err(err);
                                },
                                Ok(unsupported) => packet = Some(unsupported.into()),
                            }
                        }
                        
                        Ok(packet.unwrap())
                    }
                }
            };
            
            //eprintln!("ENUM: {output}");
            output
        }
        _ => panic!("derive wrapper only supports enums"),
    }
}

fn for_each_variant(e: &DataEnum, arm: &proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
    let mut encode_variants = Vec::with_capacity(e.variants.len());
    for variant in &e.variants {
        let ident = &variant.ident;
        
        encode_variants.push(quote! {
            Self::#ident(p) => #arm,
        });
    }
    
    encode_variants
}
