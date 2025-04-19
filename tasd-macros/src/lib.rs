use proc_macro::TokenStream;

mod packet;
mod wrapper;

#[proc_macro_derive(Packet, attributes(key, u8_enum, u16_enum, u8_string, remaining_vec))]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    packet::derive_packet_inner(input)
}

#[proc_macro_derive(Wrapper, attributes(unsupported))]
pub fn derive_wrapper(input: TokenStream) -> TokenStream {
    wrapper::derive_wrapper_inner(input)
}