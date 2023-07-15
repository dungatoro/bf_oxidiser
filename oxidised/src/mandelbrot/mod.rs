pub fn mandelbrot() {

let mut tape: [u8;30000] = [0;30000];
let mut p: usize = 0;

tape[p] = tape[p].wrapping_add(13);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(2);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(5);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(2);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 
}

p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(6);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(253);
p += 10; p %= 30000; 
tape[p] = tape[p].wrapping_add(15);

while tape[p] != 0 {

while tape[p] != 0 {
p += 9; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(5);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
}

p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 26; p %= 30000; 
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 509983; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 2; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 2; p %= 30000; 
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(4);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
}

p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(7);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
}

p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 479984; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 6; p %= 30000; 

while tape[p] != 0 {
p += 5; p %= 30000; 
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 
}

p += 179994; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 
}

p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 209993; p %= 30000; 
p += 59998; p %= 30000; 
}

p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 
}

p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(15);

while tape[p] != 0 {

while tape[p] != 0 {
p += 9; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 209993; p %= 30000; 
p += 59998; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 299990; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 4; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 5; p %= 30000; 
p += 4; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 
}

p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 2; p %= 30000; 
p += 7; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 329989; p %= 30000; 
}

p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 4; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
p += 29999; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1079964; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 13; p %= 30000; 
p += 23; p %= 30000; 
}

p += 5; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(15);

while tape[p] != 0 {

while tape[p] != 0 {
p += 4; p %= 30000; 
p += 5; p %= 30000; 
}

p += 269991; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 21; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
p += 179994; p %= 30000; 
p += 89997; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 389987; p %= 30000; 

while tape[p] != 0 {
p += 149995; p %= 30000; 
p += 119996; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 359988; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 179994; p %= 30000; 
p += 59998; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(19);
tape[p] = tape[p].wrapping_add(7);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 59998; p %= 30000; 
}

p += 2; p %= 30000; 

while tape[p] != 0 {
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
}

p += 3; p %= 30000; 
}

p += 13; p %= 30000; 

while tape[p] != 0 {
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 5; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 6; p %= 30000; 

while tape[p] != 0 {
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 239992; p %= 30000; 
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 9; p %= 30000; 
}

p += 7; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(15);

while tape[p] != 0 {

while tape[p] != 0 {
p += 9; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 89997; p %= 30000; 
p += 179994; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 
}

p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 29999; p %= 30000; 
p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 299990; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 3; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 6; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 
}

p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 
}

p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 299990; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 4; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1079964; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 36; p %= 30000; 
}

p += 5; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1079964; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 
p += 35; p %= 30000; 
}

p += 6; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(8);
tape[p] = tape[p].wrapping_add(7);

while tape[p] != 0 {

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 269991; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 
}

p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 6; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 3; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(2);
p += 119996; p %= 30000; 
}

p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 
}

p += 6; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 359988; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 329989; p %= 30000; 

while tape[p] != 0 {
p += 149995; p %= 30000; 
p += 119996; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 149995; p %= 30000; 
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 419986; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 269991; p %= 30000; 
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 
}

p += 59998; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 419986; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 59998; p %= 30000; 
p += 299990; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 119996; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 89997; p %= 30000; 
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 419986; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 10; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
}

p += 1; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 10; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 419986; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 10; p %= 30000; 
}

p += 1; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
}

p += 329989; p %= 30000; 
}

p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 
}

}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 419986; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 419986; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 
}

p += 59998; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 419986; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 209993; p %= 30000; 
p += 149995; p %= 30000; 
}

}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 5; p %= 30000; 

while tape[p] != 0 {
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 6; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(15);

while tape[p] != 0 {

while tape[p] != 0 {
p += 9; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 299990; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 3; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1079964; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 36; p %= 30000; 
}

p += 1; p %= 30000; 
p += 5; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(15);

while tape[p] != 0 {

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 269991; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
p += 119996; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 
p += 239992; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 359988; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 2; p %= 30000; 
p += 7; p %= 30000; 
}

p += 239992; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 239992; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 299990; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 12; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 299990; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 12; p %= 30000; 
}

p += 89997; p %= 30000; 
}

p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 299990; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 12; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 389987; p %= 30000; 
}

}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 2; p %= 30000; 
p += 6; p %= 30000; 
}

p += 239992; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 299990; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 
}

p += 1; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 299990; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
p += 7; p %= 30000; 
}

p += 59998; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 299990; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 
}

p += 1; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 
}

p += 359988; p %= 30000; 
}

p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 4; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 5; p %= 30000; 

while tape[p] != 0 {
p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 
}

p += 179994; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
p += 1; p %= 30000; 
}

p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(2);
p += 119996; p %= 30000; 
}

p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
p += 89997; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 29999; p %= 30000; 
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 359988; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 29999; p %= 30000; 
p += 209993; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 239992; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 389987; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 10; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 1; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 389987; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 10; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 389987; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 10; p %= 30000; 
}

p += 1; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 329989; p %= 30000; 
}

p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 
}

p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 
}

}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 
p += 3; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 239992; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 389987; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 389987; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 
}

p += 59998; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 389987; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 359988; p %= 30000; 
}

}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 119996; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
p += 2; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 
}

p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 6; p %= 30000; 
p += 3; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 389987; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 239992; p %= 30000; 
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 
}

p += 59998; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 389987; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 11; p %= 30000; 
}

p += 29999; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 239992; p %= 30000; 
p += 119996; p %= 30000; 
}

}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 6; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 5; p %= 30000; 

while tape[p] != 0 {
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 6; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 
}

p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(15);

while tape[p] != 0 {

while tape[p] != 0 {
p += 4; p %= 30000; 
p += 5; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
p += 6; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 299990; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 4; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 
}

p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 6; p %= 30000; 
p += 3; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 329989; p %= 30000; 
}

p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
p += 5; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 4; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1079964; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 17; p %= 30000; 
p += 19; p %= 30000; 
}

p += 5; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(15);

while tape[p] != 0 {

while tape[p] != 0 {
p += 8; p %= 30000; 
p += 1; p %= 30000; 
}

p += 269991; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 21; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 389987; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
p += 329989; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 
}

p += 1; p %= 30000; 
p += 7; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 59998; p %= 30000; 
}

p += 2; p %= 30000; 
}

p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
print!("{}", tape[p] as char);

p += 2; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
print!("{}", tape[p] as char);

p += 7; p %= 30000; 
}

p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 3; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 3; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 4; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(11);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
}

p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 239992; p %= 30000; 
p += 179994; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 
}

p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 2; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 149995; p %= 30000; 
p += 119996; p %= 30000; 

while tape[p] != 0 {
p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 
}

p += 179994; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 209993; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 119996; p %= 30000; 
p += 179994; p %= 30000; 
}

}

p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 
}

p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 4; p %= 30000; 
}

p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 3; p %= 30000; 
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 119996; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 209993; p %= 30000; 

while tape[p] != 0 {
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 419986; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 149995; p %= 30000; 
p += 119996; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 7; p %= 30000; 
}

p += 179994; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 

while tape[p] != 0 {
p += 2; p %= 30000; 
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 4; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 3; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 7; p %= 30000; 
}

p += 59998; p %= 30000; 
p += 119996; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(5);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
}

p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
p += 59998; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 
}

p += 119996; p %= 30000; 
p += 89997; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 479984; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 7; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 
}

p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 419986; p %= 30000; 

while tape[p] != 0 {
p += 89997; p %= 30000; 
p += 179994; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 209993; p %= 30000; 
p += 59998; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(5);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
}

p += 4; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 

while tape[p] != 0 {
p += 209993; p %= 30000; 
p += 59998; p %= 30000; 
}

}

p += 3; p %= 30000; 
}

p += 119996; p %= 30000; 
print!("{}", tape[p] as char);

p += 10; p %= 30000; 

while tape[p] != 0 {
p += 6; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 3; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(10);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 8; p %= 30000; 
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
}

p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 449985; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 239992; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 8; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 1; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 209993; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 
p += 1; p %= 30000; 
}

p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 239992; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 299990; p %= 30000; 
}

}

p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 239992; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 8; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
}

p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 149995; p %= 30000; 
}

p += 6; p %= 30000; 
p += 2; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 239992; p %= 30000; 

while tape[p] != 0 {
p += 6; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 449985; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 239992; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 209993; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 209993; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 
p += 89997; p %= 30000; 
}

p += 8; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 89997; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 239992; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 8; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 239992; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 8; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 3; p %= 30000; 
p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 59998; p %= 30000; 
}

p += 3; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 239992; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 2; p %= 30000; 
}

p += 29999; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 59998; p %= 30000; 
p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 7; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 209993; p %= 30000; 
}

p += 29999; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(5);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
}

p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 27; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 179994; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 

while tape[p] != 0 {
p += 6; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 6; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
p += 149995; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 2; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 239992; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 
}

p += 239992; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 8; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 509983; p %= 30000; 

while tape[p] != 0 {
p += 209993; p %= 30000; 
p += 59998; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 5; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 1; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 8; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 239992; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 8; p %= 30000; 
}

tape[p] = tape[p].wrapping_add(1);
p += 239992; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 8; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 59998; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 
}

p += 179994; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 6; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 449985; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 3; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 6; p %= 30000; 

while tape[p] != 0 {
p += 6; p %= 30000; 
p += 3; p %= 30000; 
}

p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

tape[p] = tape[p].wrapping_add(1);
p += 29999; p %= 30000; 
}

}

tape[p] = tape[p].wrapping_add(1);
p += 1; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 29999; p %= 30000; 

while tape[p] != 0 {
p += 9; p %= 30000; 
}

p += 239992; p %= 30000; 
}

p += 8; p %= 30000; 
}

p += 269991; p %= 30000; 

while tape[p] != 0 {
p += 269991; p %= 30000; 
}

p += 4; p %= 30000; 

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
}

p += 89997; p %= 30000; 
tape[p] = tape[p].wrapping_add(4);
tape[p] = tape[p].wrapping_add(1);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);

while tape[p] != 0 {
tape[p] = tape[p].wrapping_add(255);
p += 9; p %= 30000; 
tape[p] = tape[p].wrapping_add(1);
p += 269991; p %= 30000; 
}

p += 9; p %= 30000; 
}

p += 5; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 27; p %= 30000; 
tape[p] = tape[p].wrapping_add(255);
p += 179994; p %= 30000; 

while tape[p] != 0 {
p += 119996; p %= 30000; 
p += 149995; p %= 30000; 
}

}

p += 3; p %= 30000; 
}

}
