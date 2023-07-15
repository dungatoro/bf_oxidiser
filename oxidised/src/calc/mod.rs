pub fn calc() {

let mut tape: [u8;30000] = [0;30000];
let mut p: usize = 0;

tape[p] = {let mut input: [u8; 1] = [0; 1]; std::io::Read::read_exact(&mut input[..], &mut [u8]).expect(""); input[0]};

p += 1; p %= 30000; 
tape[p] = {let mut input: [u8; 1] = [0; 1]; std::io::Read::read_exact(&mut input[..], &mut [u8]).expect(""); input[0]};

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 
print!("{}", tape[p] as char);

}
