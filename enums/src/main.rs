enum KeysEnum {
    KeyOne,
    KeyTwo(char),
    KeyThree(i32),
    KeyFour { sub_key_one: i32, sub_key_two: i32 },
}

fn main() {
    let key_one = KeysEnum::KeyOne;
    let key_two = KeysEnum::KeyTwo('H');
    let key_three = KeysEnum::KeyThree(69);
    let key_four_sub_one = KeysEnum::KeyFour {
        sub_key_one: (69),
        sub_key_two: (420),
    };
}
