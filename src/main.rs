use colored::Colorize;
use crc16::{State, ARC};
use num::{BigUint, ToPrimitive};

/*
Primes
      2      3      5      7     11     13     17     19     23     29
     31     37     41     43     47     53     59     61     67     71
     73     79     83     89     97    101    103    107    109    113
    127    131    137    139    149    151    157    163    167    173
    179    181    191    193    197    199    211    223    227    229
    233    239    241    251    257    263    269    271    277    281
    283    293    307    311    313    317    331    337    347    349
    353    359    367    373    379    383    389    397    401    409
    419    421    431    433    439    443    449    457    461    463
    467    479    487    491    499    503    509    521    523    541
    547    557    563    569    571    577    587    593    599    601
    607    613    617    619    631    641    643    647    653    659
    661    673    677    683    691    701    709    719    727    733
    739    743    751    757    761    769    773    787    797    809
    811    821    823    827    829    839    853    857    859    863
    877    881    883    887    907    911    919    929    937    941
    947    953    967    971    977    983    991    997   1009   1013
   1019   1021   1031   1033   1039   1049   1051   1061   1063   1069
*/
fn main() {
    // let mut rng = rand::thread_rng();
    let p: BigUint = 211_u64.into(); // prime
    let q: BigUint = 229_u64.into(); // prime
    let kp = rsa::KeyPair::new_from_p_q(&p, &q);
    encrypt_decrypt(kp.clone());
    sign_check_sign(kp);
}

#[allow(dead_code)]
fn sign_check_sign(rsa::KeyPair { private, public }: rsa::KeyPair) {
    println!("{}: {}", "Открытый ключ".blue().bold(), &public);
    println!("{}: {}", "Закрытый ключ".bold().blue(), &private);
    let messages = [4_u8, 0, 13, 8, 11, 32, 10, 14, 13, 4, 16, 0, 18, 28, 5];
    println!(
        "Собщение для подписи: m[] = {}",
        format!("{:?}", messages).bold()
    );
    let mut hasher = State::<ARC>::new();
    hasher.update(&messages);
    let hash = hasher.get();
    let hash = BigUint::from(hash);
    assert!(public.n >= hash, "`n` должно быть больше чем сообщение");
    println!("CRC16 хеш сообщения: {}", hash.to_str_radix(10).bold());
    let s = private.sign(&hash);
    println!("Получена подпись хеша: {}", s.to_str_radix(10).bold());
    let restored_sign = public.restore_sign(&s);
    println!(
        "Восстановленое сообщение: {}",
        restored_sign.to_str_radix(10).underline()
    );
    println!(
        "Проверяем подпись открытым ключом: {}",
        if restored_sign == hash {
            "подпись правильная".green()
        } else {
            "подпись неправильная".red()
        }
    );
}
#[allow(dead_code)]
fn encrypt_decrypt(rsa::KeyPair { private, public }: rsa::KeyPair) {
    println!("{}: {}", "Открытый ключ".blue().bold(), &public);
    println!("{}: {}", "Закрытый ключ".bold().blue(), &private);

    // let one_to_n = range(1_u32.into(), public.n.clone()).collect::<Vec<_>>();
    let messages = [4_u32, 0, 13, 8, 11, 32, 10, 14, 13, 4, 16, 0, 18, 28, 5];

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
