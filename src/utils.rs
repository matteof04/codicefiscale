/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 *
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */
/// Check is the given char is a vowel or not
pub fn is_vowel(c: &char) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let c = c.to_ascii_lowercase();
    VOWELS.contains(&c)
}

/// Check is the given char is a consonant or not
pub fn is_consonant(c: &char) -> bool {
    !is_vowel(c)
}
