
use std::{thread, time};

use hidapi::HidApi;

fn main() {

    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                let vendor_id = device.vendor_id();
                let product_id = device.product_id();
                println!("{:04x}:{:04x}", vendor_id, product_id);
                if (vendor_id == 0x1294) && (product_id == 0x1320) {
                    let path = device.path();
                    let v = api.open_path(path).unwrap();

                    let product_info = v.get_product_string();
                    println!("product_info = {:?}", product_info);
                    
                    let mut color: u8 = 1;
                    let ten_millis = time::Duration::from_millis(175);

                    loop {
                        color = color + 1;
                        if color > 1 {
                            color = 0;
                        }
                        let buf: [u8; 6] = [0x0, color, 0x0, 0x0, 0x0, 0x0 ];
                        v.write(&buf);
                        //
                        thread::sleep(ten_millis);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }

}
