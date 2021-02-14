use colored::Colorize;
use num::{range, BigUint};
use rand::prelude::SliceRandom;
// Primes: 2 3 5 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 71
fn main() {
    let mut rng = rand::thread_rng();
    let p: BigUint = 71_u64.into(); // prime
    let q: BigUint = 67_u64.into(); // prime
    let rsa::KeyPair { private, public } = rsa::KeyPair::new_from_p_q(&p, &q);
    println!("{}: {}", "Открытый ключ".blue().bold(), &public);

    println!("{}: {}", "Закрытый ключ".bold().blue(), &private);

    // выбираем сообщение
    let one_to_n = range(1_u32.into(), public.n.clone()).collect::<Vec<_>>();
    let m = one_to_n
        .choose(&mut rng)
        .expect("ошибка: не могу выбрать случайно число в качестве сообщения");

    assert!(one_to_n.contains(&m), "`m` должна быть в интервале `1..n`");
    println!(
        "Выбрали в качестве сообщения число: m = {}",
        m.to_string().bold()
    );
    // шифруемем сообщение
    let c = public.encrypt(m);
    assert!(one_to_n.contains(&c), "`c` должна быть в интервале `1..n`");
    println!(
        "Зашифрованое сообщение: c = {}",
        c.to_string().underline().bold()
    );
    // расшифровуем его
    let me = private.decrypt(&c);

    println!(
        "Расшифрованое сообщение: m' = {}",
        me.to_string().bold().underline()
    );
}
