//! =====================================================
//!           exflag2 (brute force), rs edition
//!                     by Arc'blroth
//! =====================================================

use rayon::prelude::{ParallelBridge, ParallelIterator};

/// Exfiltrates a flag, character by character, given a brute-force oracle and a charset.
fn extract_flag2<Checker>(checker: Checker, prefix: &[u8], charset: &[u8]) -> String
where
    Checker: Fn(&[u8]) -> bool + Send + Sync + 'static,
{
    struct FlagGenerator<'f> {
        charset: &'f [u8],
        prefix: &'f [u8],
        flag_len: usize,
        flag: Vec<usize>,
        try_flag: Vec<u8>,
        ready_for_next_len: bool,
    }

    impl<'f> FlagGenerator<'f> {
        pub fn new(charset: &'f [u8], prefix: &'f [u8]) -> Self {
            let flag_len = 1;
            let flag = vec![0usize; flag_len];
            let mut try_flag = vec![0u8; prefix.len() + flag_len];
            try_flag[0..prefix.len()].copy_from_slice(prefix);

            Self {
                charset,
                prefix,
                flag_len,
                flag,
                try_flag,
                ready_for_next_len: false,
            }
        }
    }

    impl<'f> Iterator for FlagGenerator<'f> {
        type Item = Vec<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.ready_for_next_len {
                self.flag_len += 1;
                for c in &mut self.flag {
                    *c = 0;
                }
                self.flag.push(0);
                for c in &mut self.try_flag[self.prefix.len()..] {
                    *c = 0;
                }
                self.try_flag.push(0);

                self.ready_for_next_len = false;
            }

            self.flag
                .iter()
                .map(|x| self.charset[*x])
                .enumerate()
                .for_each(|(i, x)| self.try_flag[self.prefix.len() + i] = x);
            let out = Some(self.try_flag.clone());

            let mut update_index = self.flag.len() - 1;
            loop {
                if self.flag[update_index] < self.charset.len() - 1 {
                    self.flag[update_index] += 1;
                    break;
                } else if update_index > 0 {
                    self.flag[update_index] = 0;
                    update_index -= 1;
                } else {
                    self.ready_for_next_len = true;
                    break;
                }
            }

            out
        }
    }

    FlagGenerator::new(charset, prefix)
        .par_bridge()
        .try_for_each(|try_flag| {
            let try_flag_as_str = std::str::from_utf8(&try_flag).unwrap();
            println!("\x1b[90mTrying \"{}\"\x1b[0m", try_flag_as_str);
            if checker(&try_flag) {
                println!("\x1b[33mFound \"{}\"!\x1b[0m", try_flag_as_str);
                Err(String::from(try_flag_as_str.clone()))
            } else {
                Ok(())
            }
        })
        .unwrap_err()
}
