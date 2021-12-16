fn main() {
    let packet = parse_packet(INPUT);
    println!("part1: {}", version_sum(&packet));
    println!("part2: {}", eval_packet(&packet));
}

fn version_sum(packet: &Packet) -> u64 {
    packet.version + match &packet.packet_type {
        PacketType::Literal(_) => 0,
        PacketType::Operator { sub_packets, .. } => sub_packets
            .iter()
            .map(version_sum)
            .sum(),
    }
}

#[test]
fn test_version_sum() {
    assert_eq!(version_sum(&parse_packet("8A004A801A8002F478")), 16);
    assert_eq!(version_sum(&parse_packet("620080001611562C8802118E34")), 12);
    assert_eq!(version_sum(&parse_packet("C0015000016115A2E0802F182340")), 23);
    assert_eq!(version_sum(&parse_packet("A0016C880162017C3686B18A3D4780")), 31);
    assert_eq!(version_sum(&parse_packet(INPUT)), 953);
}

fn eval_packet(packet: &Packet) -> u64 {
    match &packet.packet_type {
        PacketType::Literal(x) => *x,
        PacketType::Operator { op_type, sub_packets } => match op_type {
          OpType::Sum => sub_packets
                .iter()
                .map(eval_packet)
                .sum(),
          OpType::Product => sub_packets
                .iter()
                .map(eval_packet)
                .product(),
          OpType::Minimum => sub_packets
                .iter()
                .map(eval_packet)
                .min()
                .unwrap(),
          OpType::Maximum => sub_packets
                .iter()
                .map(eval_packet)
                .max()
                .unwrap(),
          OpType::GreaterThan => {
              assert_eq!(sub_packets.len(), 2);
              (eval_packet(&sub_packets[0]) > eval_packet(&sub_packets[1])) as u64
          },
          OpType::LessThan => {
              assert_eq!(sub_packets.len(), 2);
              (eval_packet(&sub_packets[0]) < eval_packet(&sub_packets[1])) as u64
          },
          OpType::EqualTo => {
              assert_eq!(sub_packets.len(), 2);
              (eval_packet(&sub_packets[0]) == eval_packet(&sub_packets[1])) as u64
          },
        },
    }
}

#[test]
fn test_eval_packet() {
    assert_eq!(eval_packet(&parse_packet("C200B40A82")), 3);
    assert_eq!(eval_packet(&parse_packet("04005AC33890")), 54);
    assert_eq!(eval_packet(&parse_packet("880086C3E88112")), 7);
    assert_eq!(eval_packet(&parse_packet("CE00C43D881120")), 9);
    assert_eq!(eval_packet(&parse_packet("D8005AC2A8F0")), 1);
    assert_eq!(eval_packet(&parse_packet("F600BC2D8F")), 0);
    assert_eq!(eval_packet(&parse_packet("9C005AC2F8F0")), 0);
    assert_eq!(eval_packet(&parse_packet("9C0141080250320F1802104A08")), 1);
    assert_eq!(eval_packet(&parse_packet(INPUT)), 246225449979);
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u64,
    packet_type: PacketType,
}

#[derive(Debug, PartialEq)]
enum PacketType {
    Literal(u64),
    Operator {
        op_type: OpType,
        sub_packets: Vec<Packet>,
    }
}

#[derive(Debug, PartialEq)]
enum OpType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

fn parse_packet(hex_str: &str) -> Packet {
    let bin_chars = hex_str
        .chars()
        .map(hex_to_binary)
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>();

    let mut pos = 0;
    let packet = parse_sub_packet(&bin_chars, &mut pos);

    // Rest of packet should be zero padding due to hexadecimal
    // representation
    let remaining_bits = bin_chars.len() - pos;
    assert!(remaining_bits <= 7);
    if remaining_bits > 0 {
        let padding = parse_binary(&bin_chars, &mut pos, remaining_bits);
        assert_eq!(padding, 0);
    }
    assert_eq!(pos, bin_chars.len());

    packet
}

#[test]
fn test_parse_packet() {
    assert_eq!(parse_packet("D2FE28"), Packet {
        version: 6,
        packet_type: PacketType::Literal(2021),
    });
    assert_eq!(parse_packet("38006F45291200"), Packet {
        version: 1,
        packet_type: PacketType::Operator {
            op_type: OpType::LessThan,
            sub_packets: vec![
                Packet {
                    version: 6,
                    packet_type: PacketType::Literal(10),
                },
                Packet {
                    version: 2,
                    packet_type: PacketType::Literal(20),
                },
            ],
        },
    });
    assert_eq!(parse_packet("EE00D40C823060"), Packet {
        version: 7,
        packet_type: PacketType::Operator {
            op_type: OpType::Maximum,
            sub_packets: vec![
                Packet {
                    version: 2,
                    packet_type: PacketType::Literal(1),
                },
                Packet {
                    version: 4,
                    packet_type: PacketType::Literal(2),
                },
                Packet {
                    version: 1,
                    packet_type: PacketType::Literal(3),
                },
            ],
        },
    });
}

fn hex_to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("unknown char {}", c),
    }
}

fn parse_sub_packet(bin_chars: &[char], pos: &mut usize) -> Packet {
    // Version is first 3 bits
    let version = parse_binary(bin_chars, pos, 3);

    // Type is next 3 bits
    let packet_type = match parse_binary(bin_chars, pos, 3) {
        4 => parse_literal_packet(bin_chars, pos),
        op => parse_operator_packet(bin_chars, pos, op),
    };

    Packet { version, packet_type }
}

fn parse_binary(bin_chars: &[char], pos: &mut usize, len: usize) -> u64 {
    let bin_str = bin_chars[*pos..*pos+len].iter().collect::<String>();
    let num = u64::from_str_radix(&bin_str, 2).expect("parse binary");
    *pos += len;
    num
}

fn parse_binary_digit(bin_chars: &[char], pos: &mut usize) -> bool {
    let digit = bin_chars[*pos];
    *pos += 1;
    match digit {
        '0' => false,
        '1' => true,
        c => panic!("unknown binary digit: {}", c),
    }
}

fn parse_literal_packet(bin_chars: &[char], pos: &mut usize) -> PacketType {
    let mut value = 0;
    loop {
        // The binary number is padded with leading zeroes until its
        // length is a multiple of four bits, and then it is broken
        // into groups of four bits. Each group is prefixed by a 1 bit
        // except the last group, which is prefixed by a 0 bit. These
        // groups of five bits immediately follow the packet header.
        let continue_bit = parse_binary_digit(bin_chars, pos);

        value <<= 4;
        value += parse_binary(bin_chars, pos, 4);

        if !continue_bit {
            break;
        }
    }
    PacketType::Literal(value)
}

fn parse_operator_packet(bin_chars: &[char], pos: &mut usize, op_num: u64) -> PacketType {
    let op_type = match op_num {
        0 => OpType::Sum,
        1 => OpType::Product,
        2 => OpType::Minimum,
        3 => OpType::Maximum,
        5 => OpType::GreaterThan,
        6 => OpType::LessThan,
        7 => OpType::EqualTo,
        _ => panic!("unknown op type {}", op_num),
    };

    let length_type_id = parse_binary_digit(bin_chars, pos);
    let mut sub_packets = Vec::new();
    if !length_type_id {
        // If the length type ID is 0, then the next 15 bits are a
        // number that represents the total length in bits of the
        // sub-packets contained by this packet.
        let sub_packet_bits = parse_binary(bin_chars, pos, 15);
        let start_pos: usize = *pos;

        while *pos < start_pos + sub_packet_bits as usize {
            sub_packets.push(parse_sub_packet(bin_chars, pos));
        }

    } else {
        // If the length type ID is 1, then the next 11 bits are a
        // number that represents the number of sub-packets
        // immediately contained by this packet.
        let num_sub_packets = parse_binary(bin_chars, pos, 11);

        for _ in 0..num_sub_packets {
            sub_packets.push(parse_sub_packet(bin_chars, pos));
        }
    }
    PacketType::Operator { op_type, sub_packets }
}

const INPUT: &str = "6053231004C12DC26D00526BEE728D2C013AC7795ACA756F93B524D8000AAC8FF80B3A7A4016F6802D35C7C94C8AC97AD81D30024C00D1003C80AD050029C00E20240580853401E98C00D50038400D401518C00C7003880376300290023000060D800D09B9D03E7F546930052C016000422234208CC000854778CF0EA7C9C802ACE005FE4EBE1B99EA4C8A2A804D26730E25AA8B23CBDE7C855808057C9C87718DFEED9A008880391520BC280004260C44C8E460086802600087C548430A4401B8C91AE3749CF9CEFF0A8C0041498F180532A9728813A012261367931FF43E9040191F002A539D7A9CEBFCF7B3DE36CA56BC506005EE6393A0ACAA990030B3E29348734BC200D980390960BC723007614C618DC600D4268AD168C0268ED2CB72E09341040181D802B285937A739ACCEFFE9F4B6D30802DC94803D80292B5389DFEB2A440081CE0FCE951005AD800D04BF26B32FC9AFCF8D280592D65B9CE67DCEF20C530E13B7F67F8FB140D200E6673BA45C0086262FBB084F5BF381918017221E402474EF86280333100622FC37844200DC6A8950650005C8273133A300465A7AEC08B00103925392575007E63310592EA747830052801C99C9CB215397F3ACF97CFE41C802DBD004244C67B189E3BC4584E2013C1F91B0BCD60AA1690060360094F6A70B7FC7D34A52CBAE011CB6A17509F8DF61F3B4ED46A683E6BD258100667EA4B1A6211006AD367D600ACBD61FD10CBD61FD129003D9600B4608C931D54700AA6E2932D3CBB45399A49E66E641274AE4040039B8BD2C933137F95A4A76CFBAE122704026E700662200D4358530D4401F8AD0722DCEC3124E92B639CC5AF413300700010D8F30FE1B80021506A33C3F1007A314348DC0002EC4D9CF36280213938F648925BDE134803CB9BD6BF3BFD83C0149E859EA6614A8C";
