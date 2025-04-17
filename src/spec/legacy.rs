use crate::spec::legacy::LegacyError::*;
use crate::spec::packets::{ConsoleType, InputChunk, InputMoment, Packet, PortController};
use crate::spec::TasdFile;


#[derive(Debug)]
pub enum LegacyError {
    MissingPortControllers,
    InputPortOutOfRange,
    UnsupportedControllers,
    UnsupportedConsole,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct R08 {
    pub inputs: Vec<[u8; 2]>,
}
impl TryFrom<TasdFile> for R08 {
    type Error = LegacyError;
    
    fn try_from(tasd: TasdFile) -> Result<Self, Self::Error> {
        let ports = tasd.packets.iter().filter_map(|p| match p {
            Packet::PortController(port) => Some(port),
            _ => None 
        }).cloned().collect::<Vec<PortController>>();
        
        if ports.is_empty() {
            return Err(MissingPortControllers);
        }
        
        for port in &ports {
            if port.kind != 0x0101 {
                return Err(UnsupportedControllers);
            }
        }
        
        let port_inputs: [Vec<u8>; 2] = tasd.packets.into_iter()
            .filter_map(|p| match p {
                Packet::InputChunk(chunk) => Some(chunk),
                _ => None
            })
            .fold([vec![], vec![]], |mut acc, chunk| {
                if chunk.port == 1 {
                    acc[0].extend_from_slice(&chunk.inputs);
                } else if chunk.port == 2 {
                    acc[1].extend_from_slice(&chunk.inputs);
                }
                
                acc
            });
        
        let [mut p1, mut p2] = port_inputs;
        
        if p1.len() < p2.len() {
            p1.resize(p2.len(), 0xFF);
        } else if p2.len() < p1.len() {
            p2.resize(p1.len(), 0xFF);
        }
        
        let mut inputs = Vec::with_capacity(p1.len());
        for i in 0..p1.len() {
            inputs.push([p1[i] ^ 0xFF, p2[i] ^ 0xFF]);
        }
        
        Ok(R08 { inputs })
    }
}
impl From<R08> for TasdFile {
    fn from(legacy: R08) -> Self {
        let mut tasd = TasdFile::new();
        
        tasd.packets.push(ConsoleType { kind: 0x01, custom: None }.into());
        
        let mut p1 = Vec::with_capacity(legacy.inputs.len());
        let mut p2 = Vec::with_capacity(legacy.inputs.len());
        
        for input in legacy.inputs {
            p1.push(input[0] ^ 0xFF);
            p2.push(input[1] ^ 0xFF);
        }
        
        if !p1.is_empty() {
            tasd.packets.push(PortController {
                port: 1,
                kind: 0x0101,
            }.into());
        }
        if !p2.is_empty() {
            tasd.packets.push(PortController {
                port: 2,
                kind: 0x0101,
            }.into());
        }
        
        if !p1.is_empty() {
            tasd.packets.push(InputChunk {
                port: 1,
                inputs: p1,
            }.into());
        }
        if !p2.is_empty() {
            tasd.packets.push(InputChunk {
                port: 2,
                inputs: p2,
            }.into());
        }
        
        tasd
    }
}



#[derive(Debug, Default, Clone, PartialEq)]
pub struct Gbi {
    pub input_text: String,
    pub console_type: u8,
}
impl TryFrom<TasdFile> for Gbi {
    type Error = LegacyError;

    fn try_from(tasd: TasdFile) -> Result<Self, Self::Error> {
        let console = tasd.packets.iter().find_map(|p| match p {
            Packet::ConsoleType(console) => Some(console.kind),
            _ => None
        });
        
        let mut moments: Vec<InputMoment> = tasd.packets.into_iter()
            .filter_map(|p| match p {
                Packet::InputMoment(moment) => Some(moment),
                _ => None
            })
            .collect();
        
        moments.sort_by(|a, b| a.index.cmp(&b.index));
        
        let mut input_text = String::with_capacity(14 * moments.len());
        let console_type;
        match console.ok_or(LegacyError::UnsupportedConsole)? {
            0x05 | 0x06 => { // GB/C
                console_type = console.unwrap();
                for moment in moments {
                    for input in moment.inputs {
                        input_text.push_str(&format!("{:08X} {:04X}\n", moment.index, input ^ 0xFF));
                    }
                }
            },
            0x07 => { // GBA
                console_type = console.unwrap();
                for moment in moments {
                    for input in moment.inputs.chunks_exact(2) {
                        input_text.push_str(&format!("{:08X} {:04X}\n", moment.index, u16::from_be_bytes(input.try_into().unwrap()) ^ 0xFFFF));
                    }
                }
            },
            _ => return Err(LegacyError::UnsupportedConsole)
        }
        
        Ok(Gbi { input_text, console_type })
    }
}
impl TryFrom<Gbi> for TasdFile {
    type Error = LegacyError;
    fn try_from(legacy: Gbi) -> Result<Self, Self::Error> {
        let mut tasd = TasdFile::new();
        
        tasd.packets.push(ConsoleType { kind: legacy.console_type, custom: None }.into());
        
        todo!();
        
        Ok(tasd)
    }
}





#[cfg(test)]
mod tests {
    use crate::spec::legacy::R08;
    use crate::spec::packets::{InputChunk, Packet, PortController};
    use crate::spec::TasdFile;
    
    #[test]
    fn r08() {
        const TEST_LEN: usize = 1234;
        let mut r08_init = R08 {
            inputs: vec![[0x00, 0x00]; TEST_LEN],
        };
        r08_init.inputs[42][0] = 0xA5;
        r08_init.inputs[999][1] = 0x5A;
        
        let tasd: TasdFile = r08_init.clone().into();
        
        let ports: Vec<PortController> = tasd.packets.iter().filter_map(|p| match p {
            Packet::PortController(port) => Some(port),
            _ => None
        }).cloned().collect();
        
        assert_eq!(ports.len(), 2);
        let p1 = ports.iter().find(|p| p.port == 1).expect("port1 should exist");
        let p2 = ports.iter().find(|p| p.port == 2).expect("port2 should exist");
        assert_eq!(p1.kind, 0x0101);
        assert_eq!(p2.kind, 0x0101);
        
        let chunks: Vec<InputChunk> = tasd.packets.iter().filter_map(|p| match p {
            Packet::InputChunk(chunk) => Some(chunk),
            _ => None
        }).cloned().collect();
        
        let mut p1 = vec![];
        let mut p2 = vec![];
        for chunk in chunks {
            match chunk.port {
                1 => p1.extend_from_slice(&chunk.inputs),
                2 => p2.extend_from_slice(&chunk.inputs),
                _ => panic!("found unexpected port")
            }
        }
        assert_eq!(p1.len(), TEST_LEN);
        assert_eq!(p2.len(), TEST_LEN);
        
        assert_eq!(p1[41], 0xFF);
        assert_eq!(p1[42], 0x5A);
        assert_eq!(p1[43], 0xFF);
        
        assert_eq!(p1[999], 0xFF);
        assert_eq!(p2[42], 0xFF);
        
        assert_eq!(p2[998], 0xFF);
        assert_eq!(p2[999], 0xA5);
        assert_eq!(p2[1000], 0xFF);
        
        assert_eq!(p1[0], 0xFF);
        assert_eq!(p2[0], 0xFF);
        
        
        let r08_convert: R08 = tasd.try_into().expect("tasd should be valid");
        assert_eq!(r08_init, r08_convert);
    }
}