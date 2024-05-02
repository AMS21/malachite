// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToLowerHexString;

#[test]
pub fn test_to_lower_hex_string() {
    assert_eq!(50u32.to_lower_hex_string(), "32");
    assert_eq!((-100i32).to_lower_hex_string(), "ffffff9c");
}
