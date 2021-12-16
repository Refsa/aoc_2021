use std::collections::HashMap;

use crate::runner::Runner;
use bit_vec::BitVec;
use lazy_static::lazy_static;

lazy_static! {
    static ref HEX_TO_BIN: HashMap<u8, [u8; 4]> = {
        [
            ('0' as u8, [0, 0, 0, 0]),
            ('1' as u8, [0, 0, 0, 1]),
            ('2' as u8, [0, 0, 1, 0]),
            ('3' as u8, [0, 0, 1, 1]),
            ('4' as u8, [0, 1, 0, 0]),
            ('5' as u8, [0, 1, 0, 1]),
            ('6' as u8, [0, 1, 1, 0]),
            ('7' as u8, [0, 1, 1, 1]),
            ('8' as u8, [1, 0, 0, 0]),
            ('9' as u8, [1, 0, 0, 1]),
            ('A' as u8, [1, 0, 1, 0]),
            ('B' as u8, [1, 0, 1, 1]),
            ('C' as u8, [1, 1, 0, 0]),
            ('D' as u8, [1, 1, 0, 1]),
            ('E' as u8, [1, 1, 1, 0]),
            ('F' as u8, [1, 1, 1, 1]),
        ]
        .into()
    };
}

type Number = u64;

#[derive(Debug)]
enum PacketType {
    Literal(Number),
    OperatorFifteenBits(Vec<Box<Packet>>),
    OperatorElevenBits(Vec<Box<Packet>>),
}

#[derive(Debug)]
struct Packet {
    version: Number,
    type_id: Number,
    content: PacketType,
}

struct BitScanner<'a> {
    bits: Vec<bool>,
    total: &'a str,
    rest: &'a str,
    pos: usize,
}

impl<'a> BitScanner<'a> {
    fn new(from: &'a str) -> Self {
        Self {
            rest: &from[..],
            total: &from[..],
            pos: 0,
            bits: from
                .chars()
                .map(|e| if e == '1' { true } else { false })
                .collect(),
        }
    }

    fn take(&mut self, cnt: usize) -> &'a str {
        let part = &self.rest[..cnt];
        self.rest = &self.rest[cnt..];
        self.pos += cnt;
        part
    }

    fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
        self.rest = &self.total[pos..];
    }

    fn get_bit(&mut self, pos: usize) -> bool {
        self.bits[pos]
    }

    fn skip(&mut self, by: usize) {
        self.pos += by;
        self.rest = &self.rest[by..];
    }
}

struct Parser<'a> {
    scanner: BitScanner<'a>,
}

impl<'a> Parser<'a> {
    fn new(from: &'a str) -> Self {
        Self {
            scanner: BitScanner::new(from),
        }
    }

    fn skip_remaining(&mut self) {
        let octets = (self.scanner.pos as f32 / 4 as f32).ceil() as usize * 4;
        self.scanner.skip(octets - self.scanner.pos);
    }

    fn read_version(&mut self) -> Number {
        Number::from_str_radix(self.scanner.take(3), 2).unwrap()
    }

    fn read_type_id(&mut self) -> Number {
        Number::from_str_radix(self.scanner.take(3), 2).unwrap()
    }

    fn read_len_type_id(&mut self) -> Number {
        Number::from_str_radix(self.scanner.take(1), 2).unwrap()
    }

    fn read_literal(&mut self) -> Number {
        let cpos = self.scanner.pos;
        let mut epos = cpos;
        loop {
            epos += 5;
            if !self.scanner.get_bit(epos - 5) {
                break;
            }
        }

        let len = epos - cpos;
        let grps = len / 5;
        let mut bin = "".to_string();
        for _ in 0..grps {
            bin = format!("{}{}", bin, &self.scanner.take(5)[1..]);
        }

        Number::from_str_radix(&bin, 2).unwrap()
    }

    fn read_bits(&mut self, bits: usize) -> Number {
        let num = self.scanner.take(bits);
        Number::from_str_radix(num, 2).unwrap()
    }

    fn read_packet(&mut self, skip: bool) -> Packet {
        let version = self.read_version();
        let type_id = self.read_type_id();

        let data = match type_id {
            4 => {
                let val = self.read_literal();
                PacketType::Literal(val)
            }
            _ => {
                let len_type_id = self.read_len_type_id();
                match len_type_id {
                    0 => {
                        let len = self.read_bits(15) as usize;
                        let epos = self.scanner.pos + len - 1;
                        let mut sp = Vec::new();

                        while self.scanner.pos < epos {
                            sp.push(Box::new(self.read_packet(false)));
                        }
                        PacketType::OperatorFifteenBits(sp)
                    }
                    1 => {
                        let len = self.read_bits(11);
                        let mut sp = Vec::new();
                        for _ in 0..len {
                            sp.push(Box::new(self.read_packet(false)));
                        }
                        PacketType::OperatorElevenBits(sp)
                    }
                    _ => unreachable!(),
                }
            }
        };

        if skip {
            self.skip_remaining();
        }

        Packet {
            version: version,
            type_id: type_id,
            content: data,
        }
    }
}

#[derive(Default)]
pub struct AOC16 {
    message: Vec<u8>,
    string: String,
}

impl Runner for AOC16 {
    fn parse(&mut self, input: &Vec<String>) {
        let as_binary = input[0]
            .bytes()
            .map(|e| HEX_TO_BIN[&e])
            .flatten()
            .collect::<Vec<u8>>();
        let string = String::from_utf8(as_binary.iter().map(|e| e + 48).collect()).unwrap();

        self.message = as_binary;
        self.string = string;
    }

    fn run_p1(&self) -> usize {
        let mut parser = Parser::new(&self.string[..]);

        sum_packet_versions(&parser.read_packet(true)) as usize
    }

    fn run_p2(&self) -> usize {
        todo!()
    }
}

fn sum_packet_versions(packet: &Packet) -> Number {
    let mut sum = packet.version;

    let sub_packets = match &packet.content {
        PacketType::OperatorElevenBits(sp) => Some(sp),
        PacketType::OperatorFifteenBits(sp) => Some(sp),
        _ => None,
    };

    if let Some(sub_packets) = sub_packets {
        for sp in sub_packets {
            sum += sum_packet_versions(sp);
        }
    }

    return sum;
}

mod tests {
    use super::*;

    fn create_runner(input: &str) -> AOC16 {
        let input = vec![input.to_string()];
        let mut aoc = AOC16::default();
        aoc.parse(&input);
        aoc
    }

    #[test]
    fn test_read_literal() {
        let aoc = create_runner("D2FE28");
        assert_eq!("110100101111111000101000".to_string(), aoc.string);

        let mut parser = Parser::new(&aoc.string[..]);

        let version = parser.read_version();
        let type_id = parser.read_type_id();
        let data = parser.read_literal();
        parser.skip_remaining();

        assert_eq!(6, version);
        assert_eq!(4, type_id);
        assert_eq!(2021, data);
        assert_eq!(0, parser.scanner.rest.len());
    }

    #[test]
    fn test_read_operator() {
        let aoc = create_runner("38006F45291200");
        assert_eq!(
            "00111000000000000110111101000101001010010001001000000000".to_string(),
            aoc.string
        );

        let mut parser = Parser::new(&aoc.string[..]);

        let version = parser.read_version();
        let type_id = parser.read_type_id();
        let len_type_id = parser.read_len_type_id();

        assert_eq!(1, version);
        assert_eq!(6, type_id);
        assert_eq!(0, len_type_id);

        let len = parser.read_bits(15);
        assert_eq!(27, len);
    }

    #[test]
    fn test_read_literal_packet() {
        let aoc = create_runner("D2FE28");
        let mut parser = Parser::new(&aoc.string[..]);
        let packet = parser.read_packet(true);

        assert_eq!(6, packet.version);
        assert_eq!(4, packet.type_id);

        let content = if let PacketType::Literal(val) = packet.content {
            Some(val)
        } else {
            None
        };

        assert!(content.is_some());
        assert_eq!(2021, content.unwrap());
    }

    #[test]
    fn test_read_operator_fifteen_packet() {
        let aoc = create_runner("38006F45291200");
        let mut parser = Parser::new(&aoc.string[..]);
        let packet = parser.read_packet(true);

        assert_eq!(1, packet.version);
        assert_eq!(6, packet.type_id);

        let sub_packets = if let PacketType::OperatorFifteenBits(sb) = packet.content {
            Some(sb)
        } else {
            None
        };

        assert!(sub_packets.is_some());

        let sub_packets = sub_packets.unwrap();
        assert_eq!(2, sub_packets.len());

        let literal1 = if let PacketType::Literal(val) = sub_packets[0].content {
            Some(val)
        } else {
            None
        };
        assert!(literal1.is_some());
        assert_eq!(10, literal1.unwrap());

        let literal2 = if let PacketType::Literal(val) = sub_packets[1].content {
            Some(val)
        } else {
            None
        };
        assert!(literal2.is_some());
        assert_eq!(20, literal2.unwrap());
    }

    #[test]
    fn test_read_operator_eleven_packet() {
        let aoc = create_runner("EE00D40C823060");
        let mut parser = Parser::new(&aoc.string[..]);
        let packet = parser.read_packet(true);

        assert_eq!(7, packet.version);
        assert_eq!(3, packet.type_id);

        let sub_packets = if let PacketType::OperatorElevenBits(sb) = packet.content {
            Some(sb)
        } else {
            None
        };

        assert!(sub_packets.is_some());

        let sub_packets = sub_packets.unwrap();
        assert_eq!(3, sub_packets.len());

        let literal1 = if let PacketType::Literal(val) = sub_packets[0].content {
            Some(val)
        } else {
            None
        };
        assert!(literal1.is_some());
        assert_eq!(1, literal1.unwrap());

        let literal2 = if let PacketType::Literal(val) = sub_packets[1].content {
            Some(val)
        } else {
            None
        };
        assert!(literal2.is_some());
        assert_eq!(2, literal2.unwrap());

        let literal3 = if let PacketType::Literal(val) = sub_packets[2].content {
            Some(val)
        } else {
            None
        };
        assert!(literal3.is_some());
        assert_eq!(3, literal3.unwrap());
    }

    #[test]
    fn test_calc_version_sum_1() {
        let aoc = create_runner("8A004A801A8002F478");
        let mut parser = Parser::new(&aoc.string[..]);
        let packet = parser.read_packet(true);

        let sum = sum_packet_versions(&packet);

        assert_eq!(16, sum);
    }

    #[test]
    fn test_calc_version_sum_2() {
        let aoc = create_runner("A0016C880162017C3686B18A3D4780");
        let mut parser = Parser::new(&aoc.string[..]);
        let packet = parser.read_packet(true);

        let sum = sum_packet_versions(&packet);

        assert_eq!(31, sum);
    }
}
