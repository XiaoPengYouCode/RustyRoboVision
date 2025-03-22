// use dora_node_api::{DoraNode, Event};
use std::error::Error;

pub mod camera_driver;
use camera_driver::pic_record;

fn main() -> Result<(), Box<dyn Error>> {
    pic_record();
    // dora_node_init()?;

    Ok(())
}

// #[allow(dead_code)]
// fn dora_node_init() -> Result<(), Box<dyn Error>> {
//     let (_node, mut events) = DoraNode::init_from_env()?;

//     while let Some(event) = events.recv() {
//         if let Event::Input { id, .. } = event {
//             match id.as_str() {
//                 "race_time" => {
//                     println!("get race time:")
//                 }
//                 other => eprintln!("Received input `{other}`"),
//             }
//         }
//     }

//     Ok(())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn try_find_camera() {
//         tokio::spawn(find_device());
//     }
// }
