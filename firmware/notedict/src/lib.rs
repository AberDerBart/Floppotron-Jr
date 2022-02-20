use proc_macro::TokenStream;
use quote::quote;

const MAIN_CLOCK: f64 = 125000000.;

fn calc_period(note: u8) -> f64 {
    MAIN_CLOCK / (f64::powf(2., (note as f64 - 69.) / 12.) * 880.)
}

fn note(note: u8) -> TokenStream {
    // let note: LitInt = syn::parse::<syn::LitInt>(input)
    //     .map_err(|err| err.to_compile_error())
    //     .unwrap();
    // let note: u8 = note.base10_parse().unwrap();
    let period = calc_period(note);
    let period_pb = calc_period(note + 1);

    let div_int = f64::ceil(period / 65536.) as u8;

    let top = f64::round(period / div_int as f64) as u16;
    let top_pb = f64::round(period_pb / div_int as f64) as u16;

    let stream = quote!(
        PwmSetting {
            div_int: #div_int,
            top: #top,
            top_pb: #top_pb,
        }
    );
    println!("Period: {}", period);
    println!("top: {}", top);
    println!("div_int: {}", div_int);

    stream.into()
}

#[proc_macro]
pub fn note_dict(_input: TokenStream) -> TokenStream {
    let note = (0..128)
        .map(|i| note(i))
        .fold(String::new(), |s, n| format!("{}{},", s, n.to_string()));

    let stream: TokenStream = format!("[{}]", note).parse().unwrap();

    stream.into()
}
