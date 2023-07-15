pub fn helloworld() {

let mut tape: [u8;30000] = [0;30000];
let mut p: usize = 0;

tape[p] = tape[p].wrapping_add(7);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(10);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(2);
print!("{}", tape[p] as char);

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(3);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(10);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
print!("{}", tape[p] as char);

tape[p] = tape[p].wrapping_add(7);
print!("{}", tape[p] as char);

print!("{}", tape[p] as char);

tape[p] = tape[p].wrapping_add(3);
print!("{}", tape[p] as char);

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(8);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(246);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
print!("{}", tape[p] as char);

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(5);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(10);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(5);
print!("{}", tape[p] as char);

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(2);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(10);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(4);
print!("{}", tape[p] as char);

tape[p] = tape[p].wrapping_add(3);
print!("{}", tape[p] as char);

tape[p] = tape[p].wrapping_add(250);
print!("{}", tape[p] as char);

tape[p] = tape[p].wrapping_add(248);
print!("{}", tape[p] as char);

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(7);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(246);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(3);
print!("{}", tape[p] as char);

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(2);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(246);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(253);
print!("{}", tape[p] as char);

}
