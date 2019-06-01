use std::error::Error;
use std::path::Path;
use std::convert::TryInto;

pub fn path_to_actions(path: &Path) -> Result<Vec<proc_macro2::TokenStream>, Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new()
       // .delimiter(",")
        .has_headers(true)
        .from_path(path)?;
    let iter = rdr.records().skip(2); // Skip the first two lines
    Ok(iter.map(|record| {
        let record = record.unwrap();

        let id = record.get(0).unwrap().parse::<u32>().unwrap();

        let cast_time_1000 = record.get(38).unwrap().parse::<i32>().unwrap() * 10;
        let cast_time_s:i32 = cast_time_1000 / 1000;
        let cast_time_ms:i32 = cast_time_1000 % 1000;
        let range_i32:i32 = record.get(15).unwrap().parse::<i32>().map(|r| match r<0 {
            true => 0,
            false => r
        }).unwrap();
        let range:u32 = range_i32.try_into().unwrap();

        let mut modifiers:Vec<proc_macro2::TokenStream> = Vec::new();
        let mut target_mask = Vec::new();
        // Can target self
        match record.get(16).unwrap() {
            "TRUE" => target_mask.push(1 << 0),
            _ => ()
        }
        // Can target party
        match record.get(17).unwrap() {
            "TRUE" => target_mask.push(1 << 1),
            _ => ()
        }
        // Can target friendly
        match record.get(18).unwrap() {
            "TRUE" => target_mask.push(1 << 2),
            _ => ()
        }
        // Can target hostile
        match record.get(19).unwrap() {
            "TRUE" => target_mask.push(1 << 3),
            _ => ()
        }
        // Will deal with groups of people later (i.e. PvP)
        let target_mask:u32 = target_mask.into_iter().fold(0, |current, new_item| { current | new_item });

        match record.get(22).unwrap() {
            "TRUE" => {
                let radius = record.get(29).unwrap().parse::<u32>().unwrap();
                modifiers.push(quote! {
                    .with_target_type(ActionTarget::Area {
                        range: #range,
                        target_mask: #target_mask,
                        radius: #radius
                    })
                });
            },
            _ => {
                modifiers.push(quote! {
                    .with_target_type(ActionTarget::Direct {
                        range: #range,
                        target_mask: #target_mask
                    })
                });
            }
        }
        quote! {
            entry.insert(
                #id,
                Action::new(#id, Moment::new(#cast_time_s, #cast_time_ms))
                #(#modifiers)*
            );
        }
    }).collect())
}