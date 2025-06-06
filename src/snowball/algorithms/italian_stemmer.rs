//! Generated from italian.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 7] = &[
    Among("", -1, 7, None),
    Among("qu", 0, 6, None),
    Among("á", 0, 1, None),
    Among("é", 0, 2, None),
    Among("í", 0, 3, None),
    Among("ó", 0, 4, None),
    Among("ú", 0, 5, None),
];

static A_1: &'static [Among<Context>; 3] = &[
    Among("", -1, 3, None),
    Among("I", 0, 1, None),
    Among("U", 0, 2, None),
];

static A_2: &'static [Among<Context>; 37] = &[
    Among("la", -1, -1, None),
    Among("cela", 0, -1, None),
    Among("gliela", 0, -1, None),
    Among("mela", 0, -1, None),
    Among("tela", 0, -1, None),
    Among("vela", 0, -1, None),
    Among("le", -1, -1, None),
    Among("cele", 6, -1, None),
    Among("gliele", 6, -1, None),
    Among("mele", 6, -1, None),
    Among("tele", 6, -1, None),
    Among("vele", 6, -1, None),
    Among("ne", -1, -1, None),
    Among("cene", 12, -1, None),
    Among("gliene", 12, -1, None),
    Among("mene", 12, -1, None),
    Among("sene", 12, -1, None),
    Among("tene", 12, -1, None),
    Among("vene", 12, -1, None),
    Among("ci", -1, -1, None),
    Among("li", -1, -1, None),
    Among("celi", 20, -1, None),
    Among("glieli", 20, -1, None),
    Among("meli", 20, -1, None),
    Among("teli", 20, -1, None),
    Among("veli", 20, -1, None),
    Among("gli", 20, -1, None),
    Among("mi", -1, -1, None),
    Among("si", -1, -1, None),
    Among("ti", -1, -1, None),
    Among("vi", -1, -1, None),
    Among("lo", -1, -1, None),
    Among("celo", 31, -1, None),
    Among("glielo", 31, -1, None),
    Among("melo", 31, -1, None),
    Among("telo", 31, -1, None),
    Among("velo", 31, -1, None),
];

static A_3: &'static [Among<Context>; 5] = &[
    Among("ando", -1, 1, None),
    Among("endo", -1, 1, None),
    Among("ar", -1, 2, None),
    Among("er", -1, 2, None),
    Among("ir", -1, 2, None),
];

static A_4: &'static [Among<Context>; 4] = &[
    Among("ic", -1, -1, None),
    Among("abil", -1, -1, None),
    Among("os", -1, -1, None),
    Among("iv", -1, 1, None),
];

static A_5: &'static [Among<Context>; 3] = &[
    Among("ic", -1, 1, None),
    Among("abil", -1, 1, None),
    Among("iv", -1, 1, None),
];

static A_6: &'static [Among<Context>; 51] = &[
    Among("ica", -1, 1, None),
    Among("logia", -1, 3, None),
    Among("osa", -1, 1, None),
    Among("ista", -1, 1, None),
    Among("iva", -1, 9, None),
    Among("anza", -1, 1, None),
    Among("enza", -1, 5, None),
    Among("ice", -1, 1, None),
    Among("atrice", 7, 1, None),
    Among("iche", -1, 1, None),
    Among("logie", -1, 3, None),
    Among("abile", -1, 1, None),
    Among("ibile", -1, 1, None),
    Among("usione", -1, 4, None),
    Among("azione", -1, 2, None),
    Among("uzione", -1, 4, None),
    Among("atore", -1, 2, None),
    Among("ose", -1, 1, None),
    Among("ante", -1, 1, None),
    Among("mente", -1, 1, None),
    Among("amente", 19, 7, None),
    Among("iste", -1, 1, None),
    Among("ive", -1, 9, None),
    Among("anze", -1, 1, None),
    Among("enze", -1, 5, None),
    Among("ici", -1, 1, None),
    Among("atrici", 25, 1, None),
    Among("ichi", -1, 1, None),
    Among("abili", -1, 1, None),
    Among("ibili", -1, 1, None),
    Among("ismi", -1, 1, None),
    Among("usioni", -1, 4, None),
    Among("azioni", -1, 2, None),
    Among("uzioni", -1, 4, None),
    Among("atori", -1, 2, None),
    Among("osi", -1, 1, None),
    Among("anti", -1, 1, None),
    Among("amenti", -1, 6, None),
    Among("imenti", -1, 6, None),
    Among("isti", -1, 1, None),
    Among("ivi", -1, 9, None),
    Among("ico", -1, 1, None),
    Among("ismo", -1, 1, None),
    Among("oso", -1, 1, None),
    Among("amento", -1, 6, None),
    Among("imento", -1, 6, None),
    Among("ivo", -1, 9, None),
    Among("ità", -1, 8, None),
    Among("istà", -1, 1, None),
    Among("istè", -1, 1, None),
    Among("istì", -1, 1, None),
];

static A_7: &'static [Among<Context>; 87] = &[
    Among("isca", -1, 1, None),
    Among("enda", -1, 1, None),
    Among("ata", -1, 1, None),
    Among("ita", -1, 1, None),
    Among("uta", -1, 1, None),
    Among("ava", -1, 1, None),
    Among("eva", -1, 1, None),
    Among("iva", -1, 1, None),
    Among("erebbe", -1, 1, None),
    Among("irebbe", -1, 1, None),
    Among("isce", -1, 1, None),
    Among("ende", -1, 1, None),
    Among("are", -1, 1, None),
    Among("ere", -1, 1, None),
    Among("ire", -1, 1, None),
    Among("asse", -1, 1, None),
    Among("ate", -1, 1, None),
    Among("avate", 16, 1, None),
    Among("evate", 16, 1, None),
    Among("ivate", 16, 1, None),
    Among("ete", -1, 1, None),
    Among("erete", 20, 1, None),
    Among("irete", 20, 1, None),
    Among("ite", -1, 1, None),
    Among("ereste", -1, 1, None),
    Among("ireste", -1, 1, None),
    Among("ute", -1, 1, None),
    Among("erai", -1, 1, None),
    Among("irai", -1, 1, None),
    Among("isci", -1, 1, None),
    Among("endi", -1, 1, None),
    Among("erei", -1, 1, None),
    Among("irei", -1, 1, None),
    Among("assi", -1, 1, None),
    Among("ati", -1, 1, None),
    Among("iti", -1, 1, None),
    Among("eresti", -1, 1, None),
    Among("iresti", -1, 1, None),
    Among("uti", -1, 1, None),
    Among("avi", -1, 1, None),
    Among("evi", -1, 1, None),
    Among("ivi", -1, 1, None),
    Among("isco", -1, 1, None),
    Among("ando", -1, 1, None),
    Among("endo", -1, 1, None),
    Among("Yamo", -1, 1, None),
    Among("iamo", -1, 1, None),
    Among("avamo", -1, 1, None),
    Among("evamo", -1, 1, None),
    Among("ivamo", -1, 1, None),
    Among("eremo", -1, 1, None),
    Among("iremo", -1, 1, None),
    Among("assimo", -1, 1, None),
    Among("ammo", -1, 1, None),
    Among("emmo", -1, 1, None),
    Among("eremmo", 54, 1, None),
    Among("iremmo", 54, 1, None),
    Among("immo", -1, 1, None),
    Among("ano", -1, 1, None),
    Among("iscano", 58, 1, None),
    Among("avano", 58, 1, None),
    Among("evano", 58, 1, None),
    Among("ivano", 58, 1, None),
    Among("eranno", -1, 1, None),
    Among("iranno", -1, 1, None),
    Among("ono", -1, 1, None),
    Among("iscono", 65, 1, None),
    Among("arono", 65, 1, None),
    Among("erono", 65, 1, None),
    Among("irono", 65, 1, None),
    Among("erebbero", -1, 1, None),
    Among("irebbero", -1, 1, None),
    Among("assero", -1, 1, None),
    Among("essero", -1, 1, None),
    Among("issero", -1, 1, None),
    Among("ato", -1, 1, None),
    Among("ito", -1, 1, None),
    Among("uto", -1, 1, None),
    Among("avo", -1, 1, None),
    Among("evo", -1, 1, None),
    Among("ivo", -1, 1, None),
    Among("ar", -1, 1, None),
    Among("ir", -1, 1, None),
    Among("erà", -1, 1, None),
    Among("irà", -1, 1, None),
    Among("erò", -1, 1, None),
    Among("irò", -1, 1, None),
];

static G_v: &'static [u8; 20] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 8, 2, 1];

static G_AEIO: &'static [u8; 19] = &[17, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 8, 2];

static G_CG: &'static [u8; 1] = &[17];

#[derive(Clone)]
struct Context {
    i_p2: i32,
    i_p1: i32,
    i_pV: i32,
}

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.cursor;
    'replab0: loop{
        let v_2 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            among_var = env.find_among(A_0, context);
            env.ket = env.cursor;
            match among_var {
                1 => {
                    if !env.slice_from("à") {
                        return false;
                    }
                }
                2 => {
                    if !env.slice_from("è") {
                        return false;
                    }
                }
                3 => {
                    if !env.slice_from("ì") {
                        return false;
                    }
                }
                4 => {
                    if !env.slice_from("ò") {
                        return false;
                    }
                }
                5 => {
                    if !env.slice_from("ù") {
                        return false;
                    }
                }
                6 => {
                    if !env.slice_from("qU") {
                        return false;
                    }
                }
                7 => {
                    if env.cursor >= env.limit {
                        break 'lab1;
                    }
                    env.next_char();
                }
                _ => ()
            }
            continue 'replab0;
        }
        env.cursor = v_2;
        break 'replab0;
    }
    env.cursor = v_1;
    'replab2: loop{
        let v_3 = env.cursor;
        'lab3: for _ in 0..1 {
            'golab4: loop {
                let v_4 = env.cursor;
                'lab5: loop {
                    if !env.in_grouping(G_v, 97, 249) {
                        break 'lab5;
                    }
                    env.bra = env.cursor;
                    'lab6: loop {
                        let v_5 = env.cursor;
                        'lab7: loop {
                            if !env.eq_s(&"u") {
                                break 'lab7;
                            }
                            env.ket = env.cursor;
                            if !env.in_grouping(G_v, 97, 249) {
                                break 'lab7;
                            }
                            if !env.slice_from("U") {
                                return false;
                            }
                            break 'lab6;
                        }
                        env.cursor = v_5;
                        if !env.eq_s(&"i") {
                            break 'lab5;
                        }
                        env.ket = env.cursor;
                        if !env.in_grouping(G_v, 97, 249) {
                            break 'lab5;
                        }
                        if !env.slice_from("I") {
                            return false;
                        }
                        break 'lab6;
                    }
                    env.cursor = v_4;
                    break 'golab4;
                }
                env.cursor = v_4;
                if env.cursor >= env.limit {
                    break 'lab3;
                }
                env.next_char();
            }
            continue 'replab2;
        }
        env.cursor = v_3;
        break 'replab2;
    }
    return true
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_pV = env.limit;
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_2 = env.cursor;
            'lab2: loop {
                if !env.in_grouping(G_v, 97, 249) {
                    break 'lab2;
                }
                'lab3: loop {
                    let v_3 = env.cursor;
                    'lab4: loop {
                        if !env.out_grouping(G_v, 97, 249) {
                            break 'lab4;
                        }
                        if !env.go_out_grouping(G_v, 97, 249) {
                            break 'lab4;
                        }
env.next_char();                        break 'lab3;
                    }
                    env.cursor = v_3;
                    if !env.in_grouping(G_v, 97, 249) {
                        break 'lab2;
                    }
                    if !env.go_in_grouping(G_v, 97, 249) {
                        break 'lab2;
                    }
env.next_char();                    break 'lab3;
                }
                break 'lab1;
            }
            env.cursor = v_2;
            'lab5: loop {
                if !env.eq_s(&"divan") {
                    break 'lab5;
                }
                break 'lab1;
            }
            env.cursor = v_2;
            if !env.out_grouping(G_v, 97, 249) {
                break 'lab0;
            }
            'lab6: loop {
                let v_4 = env.cursor;
                'lab7: loop {
                    if !env.out_grouping(G_v, 97, 249) {
                        break 'lab7;
                    }
                    if !env.go_out_grouping(G_v, 97, 249) {
                        break 'lab7;
                    }
env.next_char();                    break 'lab6;
                }
                env.cursor = v_4;
                if !env.in_grouping(G_v, 97, 249) {
                    break 'lab0;
                }
                if env.cursor >= env.limit {
                    break 'lab0;
                }
                env.next_char();
                break 'lab6;
            }
            break 'lab1;
        }
        context.i_pV = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    let v_5 = env.cursor;
    'lab8: loop {
        if !env.go_out_grouping(G_v, 97, 249) {
            break 'lab8;
        }
env.next_char();        if !env.go_in_grouping(G_v, 97, 249) {
            break 'lab8;
        }
env.next_char();        context.i_p1 = env.cursor;
        if !env.go_out_grouping(G_v, 97, 249) {
            break 'lab8;
        }
env.next_char();        if !env.go_in_grouping(G_v, 97, 249) {
            break 'lab8;
        }
env.next_char();        context.i_p2 = env.cursor;
        break 'lab8;
    }
    env.cursor = v_5;
    return true
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            if (env.cursor >= env.limit || (env.current.as_bytes()[(env.cursor + 0) as usize] as u8 != 73 as u8 && env.current.as_bytes()[(env.cursor + 0) as usize] as u8 != 85 as u8)) {among_var = 3;}
            else {
                among_var = env.find_among(A_1, context);
            }
            env.ket = env.cursor;
            match among_var {
                1 => {
                    if !env.slice_from("i") {
                        return false;
                    }
                }
                2 => {
                    if !env.slice_from("u") {
                        return false;
                    }
                }
                3 => {
                    if env.cursor >= env.limit {
                        break 'lab1;
                    }
                    env.next_char();
                }
                _ => ()
            }
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
    }
    return true
}

fn r_RV(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_pV <= env.cursor
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor
}

fn r_attached_pronoun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((33314 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among_b(A_2, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 111 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 114 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_3, context);
    if among_var == 0 {
        return false;
    }
    if !r_RV(env, context) {
        return false;
    }
    match among_var {
        1 => {
            if !env.slice_del() {
                return false;
            }
        }
        2 => {
            if !env.slice_from("e") {
                return false;
            }
        }
        _ => ()
    }
    return true
}

fn r_standard_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_6, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
        }
        2 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
            let v_1 = env.limit - env.cursor;
            'lab0: loop {
                env.ket = env.cursor;
                if !env.eq_s_b(&"ic") {
                    env.cursor = env.limit - v_1;
                    break 'lab0;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_1;
                    break 'lab0;
                }
                if !env.slice_del() {
                    return false;
                }
                break 'lab0;
            }
        }
        3 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.slice_from("log") {
                return false;
            }
        }
        4 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.slice_from("u") {
                return false;
            }
        }
        5 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.slice_from("ente") {
                return false;
            }
        }
        6 => {
            if !r_RV(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
        }
        7 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
            let v_2 = env.limit - env.cursor;
            'lab1: loop {
                env.ket = env.cursor;
                if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((4722696 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }

                among_var = env.find_among_b(A_4, context);
                if among_var == 0 {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }
                if !env.slice_del() {
                    return false;
                }
                match among_var {
                    1 => {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"at") {
                            env.cursor = env.limit - v_2;
                            break 'lab1;
                        }
                        env.bra = env.cursor;
                        if !r_R2(env, context) {
                            env.cursor = env.limit - v_2;
                            break 'lab1;
                        }
                        if !env.slice_del() {
                            return false;
                        }
                    }
                    _ => ()
                }
                break 'lab1;
            }
        }
        8 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
            let v_3 = env.limit - env.cursor;
            'lab2: loop {
                env.ket = env.cursor;
                if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((4198408 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                    env.cursor = env.limit - v_3;
                    break 'lab2;
                }

                if env.find_among_b(A_5, context) == 0 {
                    env.cursor = env.limit - v_3;
                    break 'lab2;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_3;
                    break 'lab2;
                }
                if !env.slice_del() {
                    return false;
                }
                break 'lab2;
            }
        }
        9 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
            let v_4 = env.limit - env.cursor;
            'lab3: loop {
                env.ket = env.cursor;
                if !env.eq_s_b(&"at") {
                    env.cursor = env.limit - v_4;
                    break 'lab3;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_4;
                    break 'lab3;
                }
                if !env.slice_del() {
                    return false;
                }
                env.ket = env.cursor;
                if !env.eq_s_b(&"ic") {
                    env.cursor = env.limit - v_4;
                    break 'lab3;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_4;
                    break 'lab3;
                }
                if !env.slice_del() {
                    return false;
                }
                break 'lab3;
            }
        }
        _ => ()
    }
    return true
}

fn r_verb_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_pV {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_pV;
    env.ket = env.cursor;
    if env.find_among_b(A_7, context) == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    if !env.slice_del() {
        return false;
    }
    env.limit_backward = v_1;
    return true
}

fn r_vowel_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        if !env.in_grouping_b(G_AEIO, 97, 242) {
            env.cursor = env.limit - v_1;
            break 'lab0;
        }
        env.bra = env.cursor;
        if !r_RV(env, context) {
            env.cursor = env.limit - v_1;
            break 'lab0;
        }
        if !env.slice_del() {
            return false;
        }
        env.ket = env.cursor;
        if !env.eq_s_b(&"i") {
            env.cursor = env.limit - v_1;
            break 'lab0;
        }
        env.bra = env.cursor;
        if !r_RV(env, context) {
            env.cursor = env.limit - v_1;
            break 'lab0;
        }
        if !env.slice_del() {
            return false;
        }
        break 'lab0;
    }
    let v_2 = env.limit - env.cursor;
    'lab1: loop {
        env.ket = env.cursor;
        if !env.eq_s_b(&"h") {
            env.cursor = env.limit - v_2;
            break 'lab1;
        }
        env.bra = env.cursor;
        if !env.in_grouping_b(G_CG, 99, 103) {
            env.cursor = env.limit - v_2;
            break 'lab1;
        }
        if !r_RV(env, context) {
            env.cursor = env.limit - v_2;
            break 'lab1;
        }
        if !env.slice_del() {
            return false;
        }
        break 'lab1;
    }
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p2: 0,
        i_p1: 0,
        i_pV: 0,
    };
    let v_1 = env.cursor;
    r_prelude(env, context);
    env.cursor = v_1;
    r_mark_regions(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_2 = env.limit - env.cursor;
    r_attached_pronoun(env, context);
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_4 = env.limit - env.cursor;
            'lab2: loop {
                if !r_standard_suffix(env, context) {
                    break 'lab2;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_4;
            if !r_verb_suffix(env, context) {
                break 'lab0;
            }
            break 'lab1;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_3;
    let v_5 = env.limit - env.cursor;
    r_vowel_suffix(env, context);
    env.cursor = env.limit - v_5;
    env.cursor = env.limit_backward;
    let v_6 = env.cursor;
    r_postlude(env, context);
    env.cursor = v_6;
    return true
}
