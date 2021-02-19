use colored::Colorize;
use num::{range, BigUint, ToPrimitive};
// use rand::prelude::SliceRandom;
// Primes: 2 3 5 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 71
fn main() {
    // let mut rng = rand::thread_rng();
    let p: BigUint = 71_u64.into(); // prime
    let q: BigUint = 67_u64.into(); // prime
    let rsa::KeyPair { private, public } = rsa::KeyPair::new_from_p_q(&p, &q);

    println!("{}: {}", "Открытый ключ".blue().bold(), &public);

    println!("{}: {}", "Закрытый ключ".bold().blue(), &private);

    // let one_to_n = range(1_u32.into(), public.n.clone()).collect::<Vec<_>>();
    let messages = [4_u32, 0, 13, 8, 11, 32, 10, 14, 13, 4, 16, 0, 18, 28, 5];

    // for msg in &messages {
    // let msg = BigUint::from(*msg);
    // assert!(one_to_n.contains(&msg), format!("msg = {}", msg.to_str_radix(10)));
    // }

    // выбираем сообщение

    println!(
        "Собщение для зашифровки: m[] = {}",
        format!("{:?}", messages).bold()
    );
    // шифруемем сообщение
    let cs = messages
        .iter()
        .map(|x| public.encrypt(&BigUint::from(*x)))
        .collect::<Vec<_>>();
    // for c in cs.iter() {
    // assert!(one_to_n.contains(&c), "`c` должна быть в интервале `1..n`");
    // }
    println!(
        "Зашифрованое сообщение: c[] = {}",
        format!(
            "{:?}",
            cs.iter().map(|x| x.to_u64().unwrap()).collect::<Vec<_>>()
        )
        .bold()
    );
    // расшифровуем его
    let md = cs.iter().map(|x| private.decrypt(x)).collect::<Vec<_>>();

    println!(
        "Расшифрованое сообщение: m' = {}",
        format!(
            "{:?}",
            md.iter().map(|x| x.to_u64().unwrap()).collect::<Vec<_>>()
        )
        .bold()
    );
}
