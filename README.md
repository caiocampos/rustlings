<!--
This file was autogenerated by the script in src/bin/generate_readme.rs.
Please edit either the script or the template in README-template.md in
order to make changes here rather than committing the changes directly.
-->

# rustlings

Small exercises to get you used to reading and writing Rust code. Includes practice reading and
responding to compiler messages!

This repo is very much the smallest thing that could possibly work :)

## To do these exercises

Thanks to [btbytes'](https://twitter.com/btbytes) [prlinks](https://github.com/btbytes/prlink), you
can now click on the links below to load the exercises in the rust playground!

There are infinite correct answers-- the exercises are sometimes left very open-ended. Scroll down
in the playground to find comments that have hints.

If you need more help or would like to compare solutions, you can ask in [#rust-beginners on
irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners ), the
[user forum](https://users.rust-lang.org/), or [the subreddit](https://reddit.com/r/rust). If an
exercise could be improved in any way, please [create an
issue](https://github.com/carols10cents/rustlings/issues/new) or submit a pull request!

### Variable bindings

[Relevant chapter in The Rust Programming
Language](https://doc.rust-lang.org/book/second-edition/ch03-01-variables-and-mutability.html)

- ["variables1.rs"](https://play.rust-lang.org/?code=%2F%2F+variables1.rs%0A%0Afn+main%28%29+%7B%0A++++let+x+%3D+5%3B%0A++++println%21%28%22x+has+the+value+%7B%7D%22%2C+x%29%3B%0A%7D)
- ["variables2.rs"](https://play.rust-lang.org/?code=%2F%2F+variables2.rs%0A%0Afn+main%28%29+%7B%0A++++let+x+%3D+0%3B%0A++++if+x+%3D%3D+10+%7B%0A++++++++println%21%28%22Ten%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Not+ten%21%22%29%3B%0A++++%7D%0A%7D)
- ["variables3.rs"](https://play.rust-lang.org/?code=%2F%2F+variables3.rs%0Afn+main%28%29+%7B%0A++++let+mut+x+%3D+3%3B%0A++++println%21%28%22Number+%7B%7D%22%2C+x%29%3B%0A++++x+%3D+5%3B%0A++++println%21%28%22Number+%7B%7D%22%2C+x%29%3B%0A%7D)
- ["variables4.rs"](https://play.rust-lang.org/?code=%2F%2F+variables4.rs%0A%0Afn+main%28%29+%7B%0A++++let+x%3A+i32+%3D+0%3B%0A++++println%21%28%22Number+%7B%7D%22%2C+x%29%3B%0A%7D)

### Functions

[Relevant chapter in The Rust Programming
Language](https://doc.rust-lang.org/book/second-edition/ch03-03-how-functions-work.html)

- ["functions1.rs"](https://play.rust-lang.org/?code=%2F%2F+functions1.rs%0A%0Afn+main%28%29+%7B%0A++++call_me%28%29%3B%0A%7D%0A%0Afn+call_me%28%29+%7B%0A++++println%21%28%22Helllo%21%21%22%29%3B%0A%7D)
- ["functions2.rs"](https://play.rust-lang.org/?code=%2F%2F+functions2.rs%0A%0Afn+main%28%29+%7B%0A++++call_me%283%29%3B%0A%7D%0A%0Afn+call_me%28num%3A+i32%29+%7B%0A++++for+i+in+0..num+%7B%0A++++++++println%21%28%22Ring%21+Call+number+%7B%7D%22%2C+i+%2B+1%29%3B%0A++++%7D%0A%7D)
- ["functions3.rs"](https://play.rust-lang.org/?code=%2F%2F+functions3.rs%0A%0Afn+main%28%29+%7B%0A++++call_me%2810%29%3B%0A%7D%0A%0Afn+call_me%28num%3A+i32%29+%7B%0A++++for+i+in+0..num+%7B%0A++++++++println%21%28%22Ring%21+Call+number+%7B%7D%22%2C+i+%2B+1%29%3B%0A++++%7D%0A%7D)
- ["functions4.rs"](https://play.rust-lang.org/?code=%2F%2F+functions4.rs%0A%0A%2F%2F+This+store+is+having+a+sale+where+if+the+price+is+an+even+number%2C+you+get%0A%2F%2F+10+%28money+unit%29+off%2C+but+if+it%27s+an+odd+number%2C+it%27s+3+%28money+unit%29+less.%0A%0Afn+main%28%29+%7B%0A++++let+original_price+%3D+51%3B%0A++++println%21%28%22Your+sale+price+is+%7B%7D%22%2C+sale_price%28original_price%29%29%3B%0A%7D%0A%0Afn+sale_price%28price%3A+i32%29+-%3E+i32+%7B%0A++++if+is_even%28price%29+%7B%0A++++++++price+-+10%0A++++%7D+else+%7B%0A++++++++price+-+3%0A++++%7D%0A%7D%0A%0Afn+is_even%28num%3A+i32%29+-%3E+bool+%7B%0A++++num+%25+2+%3D%3D+0%0A%7D)
- ["functions5.rs"](https://play.rust-lang.org/?code=%2F%2F+functions5.rs%0A%0Afn+main%28%29+%7B%0A++++let+answer+%3D+square%283%29%3B%0A++++println%21%28%22The+answer+is+%7B%7D%22%2C+answer%29%3B%0A%7D%0A%0Afn+square%28num%3A+i32%29+-%3E+i32+%7B%0A++++num+*+num%0A%7D)

### Primitive types

[Relevant chapter in The Rust Programming
Language](https://doc.rust-lang.org/book/second-edition/ch03-02-data-types.html)

- ["primitive_types1.rs"](https://play.rust-lang.org/?code=%2F%2F+primitive_types1.rs%0A%0Afn+main%28%29+%7B%0A++++%2F%2F+Booleans+%28%60bool%60%29%0A%0A++++let+is_morning+%3D+true%3B%0A++++if+is_morning+%7B%0A++++++++println%21%28%22Good+morning%21%22%29%3B%0A++++%7D%0A%0A++++let+is_evening+%3D+true%3B%0A++++if+is_evening+%7B%0A++++++++println%21%28%22Good+evening%21%22%29%3B%0A++++%7D%0A%7D)
- ["primitive_types2.rs"](https://play.rust-lang.org/?code=%2F%2F+primitive_types2.rs%0A%0Afn+main%28%29+%7B%0A++++%2F%2F+Characters+%28%60char%60%29%0A%0A++++let+my_first_initial+%3D+%27C%27%3B%0A++++if+my_first_initial.is_alphabetic%28%29+%7B%0A++++++++println%21%28%22Alphabetical%21%22%29%3B%0A++++%7D+else+if+my_first_initial.is_numeric%28%29+%7B%0A++++++++println%21%28%22Numerical%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Neither+alphabetic+nor+numeric%21%22%29%3B%0A++++%7D%0A%0A++++let+your_character+%3D+%27%40%27%3B%0A++++if+your_character.is_alphabetic%28%29+%7B%0A++++++++println%21%28%22Alphabetical%21%22%29%3B%0A++++%7D+else+if+your_character.is_numeric%28%29+%7B%0A++++++++println%21%28%22Numerical%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Neither+alphabetic+nor+numeric%21%22%29%3B%0A++++%7D%0A%7D)
- ["primitive_types3.rs"](https://play.rust-lang.org/?code=%2F%2F+primitive_types3.rs%0A%0Afn+main%28%29+%7B%0A++++let+a+%3D+%5B1%3B+200%5D%3B%0A%0A++++if+a.len%28%29+%3E%3D+100+%7B%0A++++++++println%21%28%22Wow%2C+that%27s+a+big+array%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Meh%2C+I+eat+arrays+like+that+for+breakfast.%22%29%3B%0A++++%7D%0A%7D)
- ["primitive_types4.rs"](https://play.rust-lang.org/?code=%2F%2F+primitive_types4.rs%0A%0Afn+main%28%29+%7B%0A++++let+a+%3D+%5B1%2C+2%2C+3%2C+4%2C+5%5D%3B%0A%0A++++let+nice_slice+%3D+%26a%5B1..%3D3%5D%3B%0A++++%2F%2F+let+nice_slice+%3D+%26a%5B1..4%5D%3B%0A%0A++++if+nice_slice+%3D%3D+%5B2%2C+3%2C+4%5D+%7B%0A++++++++println%21%28%22Nice+slice%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22Not+quite+what+I+was+expecting...+I+see%3A+%7B%3A%3F%7D%22%2C+nice_slice%29%3B%0A++++%7D%0A%7D)
- ["primitive_types5.rs"](https://play.rust-lang.org/?code=%2F%2F+primitive_types5.rs%0A%0Afn+main%28%29+%7B%0A++++let+cat+%3D+%28%22Furry+McFurson%22%2C+3.5%29%3B%0A++++let+%28name%2C+age%29+%3D+cat%3B%0A%0A++++println%21%28%22%7B%7D+is+%7B%7D+years+old.%22%2C+name%2C+age%29%3B%0A%7D)
- ["primitive_types6.rs"](https://play.rust-lang.org/?code=%2F%2F+primitive_types6.rs%0A%0Afn+main%28%29+%7B%0A++++let+numbers+%3D+%281%2C+2%2C+3%29%3B%0A++++println%21%28%22The+second+number+is+%7B%7D%22%2C+numbers.1%29%3B%0A%7D)

### Tests

Going out of order from the book to cover tests-- many of the following exercises will ask you to
make tests pass!

[Relevant chapter in The Rust Programming
Language](https://doc.rust-lang.org/book/second-edition/ch11-01-writing-tests.html)

- ["tests1.rs"](https://play.rust-lang.org/?code=%2F%2F+tests1.rs%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++%23%5Btest%5D%0A++++fn+you_can_assert%28%29+%7B%0A++++++++assert%21%2810+%3D%3D+10%29%3B%0A++++%7D%0A%7D)
- ["tests2.rs"](https://play.rust-lang.org/?code=%2F%2F+tests2.rs%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++%23%5Btest%5D%0A++++fn+you_can_assert_eq%28%29+%7B%0A++++++++assert_eq%21%281+%2B+2%2C+3%29%3B%0A++++%7D%0A%7D)
- ["tests3.rs"](https://play.rust-lang.org/?code=%2F%2F+tests3.rs%0A%0Apub+fn+is_even%28num%3A+i32%29+-%3E+bool+%7B%0A++++num+%25+2+%3D%3D+0%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+is_true_when_even%28%29+%7B%0A++++++++assert%21%28is_even%284%29%29%3B%0A++++%7D%0A%7D)
- ["tests4.rs"](https://play.rust-lang.org/?code=%2F%2F+tests4.rs%0A%0Apub+fn+times_two%28num%3A+i32%29+-%3E+i32+%7B%0A++++num+*+2%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+returns_twice_of_positive_numbers%28%29+%7B%0A++++++++assert_eq%21%28times_two%282%29%2C+4%29%3B%0A++++%7D%0A%7D%0A)

### If

[Relevant chapter in The Rust Programming
Language](https://doc.rust-lang.org/book/second-edition/ch03-05-control-flow.html)

- ["if1.rs"](https://play.rust-lang.org/?code=%2F%2F+if1.rs%0A%0Apub+fn+bigger%28a%3A+i32%2C+b%3Ai32%29+-%3E+i32+%7B%0A++++if+a+%3E+b+%7B%0A++++++++a%0A++++%7D+else+%7B%0A++++++++b%0A++++%7D%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+ten_is_bigger_than_eight%28%29+%7B%0A++++++++assert_eq%21%2810%2C+bigger%2810%2C+8%29%29%3B%0A++++%7D%0A%0A++++%23%5Btest%5D%0A++++fn+fortytwo_is_bigger_than_thirtytwo%28%29+%7B%0A++++++++assert_eq%21%2842%2C+bigger%2832%2C+42%29%29%3B%0A++++%7D%0A%7D)

### Strings

[Relevant chapter in The Rust Programming
Language](https://doc.rust-lang.org/book/second-edition/ch08-02-strings.html)

- ["strings1.rs"](https://play.rust-lang.org/?code=%2F%2F+strings1.rs%0A%0Afn+main%28%29+%7B%0A++++let+answer+%3D+current_favorite_color%28%29%3B%0A++++println%21%28%22My+current+favorite+color+is+%7B%7D%22%2C+answer%29%3B%0A%7D%0A%0Afn+current_favorite_color%28%29+-%3E+String+%7B%0A++++String%3A%3Afrom%28%22blue%22%29%0A%7D)
- ["strings2.rs"](https://play.rust-lang.org/?code=%2F%2F+strings2.rs%0A%0Afn+main%28%29+%7B%0A++++let+word+%3D+String%3A%3Afrom%28%22green%22%29%3B%0A++++if+is_a_color_word%28%26word%29+%7B%0A++++++++println%21%28%22That+is+a+color+word+I+know%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22That+is+not+a+color+word+I+know.%22%29%3B%0A++++%7D%0A%7D%0A%0Afn+is_a_color_word%28attempt%3A+%26str%29+-%3E+bool+%7B%0A++++attempt+%3D%3D+%22green%22+%7C%7C+attempt+%3D%3D+%22blue%22+%7C%7C+attempt+%3D%3D+%22red%22%0A%7D)
- ["strings3.rs"](https://play.rust-lang.org/?code=%2F%2F+strings3.rs%0A%0Afn+string_slice%28arg%3A+%26str%29+%7B+println%21%28%22%7B%7D%22%2C+arg%29%3B+%7D%0Afn+string%28arg%3A+String%29+%7B+println%21%28%22%7B%7D%22%2C+arg%29%3B+%7D%0A%0Afn+main%28%29+%7B%0A++++string_slice%28%22blue%22%29%3B%0A++++string%28%22red%22.to_string%28%29%29%3B%0A++++string%28String%3A%3Afrom%28%22hi%22%29%29%3B%0A++++string%28%22rust+is+fun%21%22.to_owned%28%29%29%3B%0A++++string%28%22nice+weather%22.into%28%29%29%3B%0A++++string%28format%21%28%22Interpolation+%7B%7D%22%2C+%22Station%22%29%29%3B%0A++++string_slice%28%26String%3A%3Afrom%28%22abc%22%29%5B0..1%5D%29%3B%0A++++string_slice%28%22++hello+there+%22.trim%28%29%29%3B%0A++++string%28%22Happy+Monday%21%22.to_string%28%29.replace%28%22Mon%22%2C+%22Tues%22%29%29%3B%0A++++string%28%22mY+sHiFt+KeY+iS+sTiCkY%22.to_lowercase%28%29%29%3B%0A%7D)

### Move semantics

These exercises are adapted from [pnkfelix]()'s [Rust
Tutorial](https://pnkfelix.github.io/rust-examples-icfp2014/) -- thank you Felix!!!

Relevant chapters in the book:

- [Ownership](https://doc.rust-lang.org/book/second-edition/ch04-01-what-is-ownership.html)
- [References and borrowing](https://doc.rust-lang.org/book/second-edition/ch04-02-references-and-borrowing.html)

Note that the exercises in this section may look similar to each other but they are subtly
different :)

- ["move_semantics1.rs"](https://play.rust-lang.org/?code=%2F%2F+move_semantics1.rs%0A%0Afn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+mut+vec1+%3D+fill_vec%28vec0%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28vec%3A+Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++let+mut+vec+%3D+vec%3B%0A%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D)
- ["move_semantics2.rs"](https://play.rust-lang.org/?code=%2F%2F+move_semantics2.rs%0A%0Afn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+mut+vec1+%3D+fill_vec%28%26vec0%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec0%22%2C+vec0.len%28%29%2C+vec0%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%7D%0A%0Afn+fill_vec%28vec%3A+%26Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++let+mut+vec+%3D+vec.clone%28%29%3B%0A%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D)
- ["move_semantics3.rs"](https://play.rust-lang.org/?code=%2F%2F+move_semantics3.rs%0A%0Afn+main%28%29+%7B%0A++++let+vec0+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++let+mut+vec1+%3D+fill_vec%28%26mut+vec0.clone%28%29%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28vec%3A+%26mut+Vec%3Ci32%3E%29+-%3E+Vec%3Ci32%3E+%7B%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec.to_vec%28%29%0A%7D)
- ["move_semantics4.rs"](https://play.rust-lang.org/?code=%2F%2F+move_semantics4.rs%0A%0Afn+main%28%29+%7B%0A++++let+mut+vec1+%3D+fill_vec%28%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A++++vec1.push%2888%29%3B%0A%0A++++println%21%28%22%7B%7D+has+length+%7B%7D+content+%60%7B%3A%3F%7D%60%22%2C+%22vec1%22%2C+vec1.len%28%29%2C+vec1%29%3B%0A%0A%7D%0A%0Afn+fill_vec%28%29+-%3E+Vec%3Ci32%3E+%7B%0A++++let+mut+vec+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++vec.push%2822%29%3B%0A++++vec.push%2844%29%3B%0A++++vec.push%2866%29%3B%0A%0A++++vec%0A%7D)

### Modules

[Relevant chapter in The Rust Programming
Language](https://doc.rust-lang.org/book/second-edition/ch07-01-mod-and-the-filesystem.html)

- ["modules1.rs"](https://play.rust-lang.org/?code=%2F%2F+modules1.rs%0A%0Amod+sausage_factory+%7B%0A++++pub+fn+make_sausage%28%29+%7B%0A++++++++println%21%28%22sausage%21%22%29%3B%0A++++%7D%0A%7D%0A%0Afn+main%28%29+%7B%0A++++sausage_factory%3A%3Amake_sausage%28%29%3B%0A%7D)
- ["modules2.rs"](https://play.rust-lang.org/?code=%2F%2F+modules2.rs%0A%0Amod+us_presidential_frontrunners+%7B%0A++++pub+use+self%3A%3Ademocrats%3A%3AHILLARY_CLINTON+as+democrat%3B%0A++++pub+use+self%3A%3Arepublicans%3A%3ADONALD_TRUMP+as+republican%3B%0A%0A++++mod+democrats+%7B%0A++++++++pub+const+HILLARY_CLINTON%3A+%26%27static+str+%3D+%22Hillary+Clinton%22%3B%0A++++++++pub+const+BERNIE_SANDERS%3A+%26%27static+str+%3D+%22Bernie+Sanders%22%3B%0A++++%7D%0A%0A++++mod+republicans+%7B%0A++++++++pub+const+DONALD_TRUMP%3A+%26%27static+str+%3D+%22Donald+Trump%22%3B%0A++++++++pub+const+JEB_BUSH%3A+%26%27static+str+%3D+%22Jeb+Bush%22%3B%0A++++%7D%0A%7D%0A%0Afn+main%28%29+%7B%0A++++println%21%28%22candidates%3A+%7B%7D+and+%7B%7D%22%2C%0A+++++++++++++us_presidential_frontrunners%3A%3Ademocrat%2C%0A+++++++++++++us_presidential_frontrunners%3A%3Arepublican%29%3B%0A%7D)

### Macros

Check out:

- [The Macros section of the first edition of the book
  book](https://doc.rust-lang.org/book/first-edition/macros.html)
- [The Macros appendix of the second edition of the
  book](https://doc.rust-lang.org/book/second-edition/appendix-04-macros.html)
- [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)

- ["macros1.rs"](https://play.rust-lang.org/?code=%2F%2F+macros1.rs%0A%0Amacro_rules%21+my_macro+%7B%0A++++%28%29+%3D%3E+%7B%0A++++++++println%21%28%22Check+out+my+macro%21%22%29%3B%0A++++%7D%3B%0A%7D%0A%0Afn+main%28%29+%7B%0A++++my_macro%21%28%29%3B%0A%7D)
- ["macros2.rs"](https://play.rust-lang.org/?code=%2F%2F+macros2.rs%0A%0Amacro_rules%21+my_macro+%7B%0A++++%28%29+%3D%3E+%7B%0A++++++++println%21%28%22Check+out+my+macro%21%22%29%3B%0A++++%7D%3B%0A%7D%0A%0Afn+main%28%29+%7B%0A++++my_macro%21%28%29%3B%0A%7D)
- ["macros3.rs"](https://play.rust-lang.org/?code=%2F%2F+macros3.rs%0A%0Amod+macros+%7B%0A++++%23%5Bmacro_export%5D%0A++++macro_rules%21+my_macro+%7B%0A++++++++%28%29+%3D%3E+%7B%0A++++++++++++println%21%28%22Check+out+my+macro%21%22%29%3B%0A++++++++%7D%3B%0A++++%7D%0A%7D%0A%0Afn+main%28%29+%7B%0A++++my_macro%21%28%29%3B%0A%7D)
- ["macros4.rs"](https://play.rust-lang.org/?code=%2F%2F+macros4.rs%0A%0Amacro_rules%21+my_macro+%7B%0A++++%28%29+%3D%3E+%7B%0A++++++++println%21%28%22Check+out+my+macro%21%22%29%3B%0A++++%7D%3B%0A++++%28%24val%3Aexpr%29+%3D%3E+%7B%0A++++++++println%21%28%22Look+at+this+other+macro%3A+%7B%7D%22%2C+%24val%29%3B%0A++++%7D%0A%7D%0A%0Afn+main%28%29+%7B%0A++++my_macro%21%28%29%3B%0A++++my_macro%21%287777%29%3B%0A%7D)

### Error Handling

The [Error
Handling](https://doc.rust-lang.org/book/second-edition/ch09-02-recoverable-errors-with-result.html)
and [Generics](https://doc.rust-lang.org/book/second-edition/ch10-01-syntax.html) sections are
relevant.

- ["option1.rs"](https://play.rust-lang.org/?code=%2F%2F+option1.rs%0A%0Afn+main%28%29+%7B%0A++++let+mut+list+%3D+vec%21%5B3%5D%3B%0A%0A++++let+last+%3D+list.pop%28%29%3B%0A++++if+let+Some%28_%29+%3D+last+%7B%0A++++++++println%21%28%22The+last+item+in+the+list+is+%7B%3A%3F%7D%22%2C+last.unwrap%28%29%29%3B%0A++++%7D%0A++++%0A++++let+second_to_last+%3D+list.pop%28%29%3B%0A++++if+let+Some%28_%29+%3D+second_to_last+%7B%0A++++++++println%21%28%22The+second-to-last+item+in+the+list+is+%7B%3A%3F%7D%22%2C+second_to_last.unwrap%28%29%29%3B%0A++++%7D%0A%7D)
- ["result1.rs"](https://play.rust-lang.org/?code=%2F%2F+result1.rs%0A%2F%2F+Make+this+test+pass%21+Scroll+down+for+hints+%3A%29%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Astruct+PositiveNonzeroInteger%28u64%29%3B%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Aenum+CreationError+%7B%0A++++Negative%2C%0A++++Zero%2C%0A%7D%0A%0Aimpl+PositiveNonzeroInteger+%7B%0A++++fn+new%28value%3A+i64%29+-%3E+Result%3CPositiveNonzeroInteger%2C+CreationError%3E+%7B%0A++++++++if+value+%3D%3D+0+%7B%0A++++++++++++Err%28CreationError%3A%3AZero%29%0A++++++++%7D+else+if+value+%3C+0+%7B%0A++++++++++++Err%28CreationError%3A%3ANegative%29%0A++++++++%7D+else+%7B%0A++++++++++++Ok%28PositiveNonzeroInteger%28value+as+u64%29%29%0A++++++++%7D%0A++++%7D%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_creation%28%29+%7B%0A++++assert%21%28PositiveNonzeroInteger%3A%3Anew%2810%29.is_ok%28%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3ANegative%29%2C+PositiveNonzeroInteger%3A%3Anew%28-10%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3AZero%29%2C+PositiveNonzeroInteger%3A%3Anew%280%29%29%3B%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+%60PositiveNonzeroInteger%3A%3Anew%60+is+always+creating+a+new+instance+and+returning+an+%60Ok%60+result.%0A%2F%2F+It+should+be+doing+some+checking%2C+returning+an+%60Err%60+result+if+those+checks+fail%2C+and+only%0A%2F%2F+returning+an+%60Ok%60+result+if+those+checks+determine+that+everything+is...+okay+%3A%29%0A)
- ["errors1.rs"](https://play.rust-lang.org/?code=%2F%2F+errors1.rs%0A%0Apub+fn+generate_nametag_text%28name%3A+String%29+-%3E+Result%3CString%2C+String%3E+%7B%0A++++if+name.len%28%29+%3E+0+%7B%0A++++++++Ok%28format%21%28%22Hi%21+My+name+is+%7B%7D%22%2C+name%29%29%0A++++%7D+else+%7B%0A++++++++Err%28%22%60name%60+was+empty%3B+it+must+be+nonempty.%22.into%28%29%29%0A++++%7D%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+generates_nametag_text_for_a_nonempty_name%28%29+%7B%0A++++++++assert_eq%21%28%0A++++++++++++generate_nametag_text%28%22Beyonc%C3%A9%22.into%28%29%29%2C%0A++++++++++++Ok%28%22Hi%21+My+name+is+Beyonc%C3%A9%22.into%28%29%29%0A++++++++%29%3B%0A++++%7D%0A%0A++++%23%5Btest%5D%0A++++fn+explains_why_generating_nametag_text_fails%28%29+%7B%0A++++++++assert_eq%21%28%0A++++++++++++generate_nametag_text%28%22%22.into%28%29%29%2C%0A++++++++++++Err%28%22%60name%60+was+empty%3B+it+must+be+nonempty.%22.into%28%29%29%0A++++++++%29%3B%0A++++%7D%0A%7D)
- ["errors2.rs"](https://play.rust-lang.org/?code=%2F%2F+errors2.rs%0A%0Ause+std%3A%3Anum%3A%3AParseIntError%3B%0A%0Apub+fn+total_cost%28item_quantity%3A+%26str%29+-%3E+Result%3Ci32%2C+ParseIntError%3E+%7B%0A++++let+processing_fee+%3D+1%3B%0A++++let+cost_per_item+%3D+5%3B%0A++++%2F*%0A++++let+qty+%3D+match+item_quantity.parse%3A%3A%3Ci32%3E%28%29+%7B%0A++++++++Ok%28val%29+%3D%3E+val%2C%0A++++++++Err%28error%29+%3D%3E+%7B%0A++++++++++++panic%21%28error%29%0A++++++++%7D%2C%0A++++%7D%3B*%2F%0A++++let+qty+%3D+item_quantity.parse%3A%3A%3Ci32%3E%28%29%3F%3B%0A%0A++++Ok%28qty+*+cost_per_item+%2B+processing_fee%29%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+item_quantity_is_a_valid_number%28%29+%7B%0A++++++++assert_eq%21%28%0A++++++++++++total_cost%28%2234%22%29%2C%0A++++++++++++Ok%28171%29%0A++++++++%29%3B%0A++++%7D%0A%0A++++%23%5Btest%5D%0A++++fn+item_quantity_is_an_invalid_number%28%29+%7B%0A++++++++assert_eq%21%28%0A++++++++++++total_cost%28%22beep+boop%22%29.unwrap_err%28%29.to_string%28%29%2C%0A++++++++++++%22invalid+digit+found+in+string%22%0A++++++++%29%3B%0A++++%7D%0A%7D)
- ["errors3.rs"](https://play.rust-lang.org/?code=%2F%2F+errors3.rs%0A%0Ause+std%3A%3Anum%3A%3AParseIntError%3B%0A%0Afn+main%28%29+%7B%0A++++let+mut+tokens+%3D+100%3B%0A++++let+pretend_user_input+%3D+%228%22%3B%0A%0A++++let+cost+%3D+match+total_cost%28pretend_user_input%29+%7B%0A++++++++Ok%28val%29+%3D%3E+val%2C%0A++++++++Err%28error%29+%3D%3E+%7B%0A++++++++++++panic%21%28error%29%0A++++++++%7D%2C%0A++++%7D%3B%0A%0A++++if+cost+%3E+tokens+%7B%0A++++++++println%21%28%22You+can%27t+afford+that+many%21%22%29%3B%0A++++%7D+else+%7B%0A++++++++tokens+-%3D+cost%3B%0A++++++++println%21%28%22You+now+have+%7B%7D+tokens.%22%2C+tokens%29%3B%0A++++%7D%0A%7D%0A%0Apub+fn+total_cost%28item_quantity%3A+%26str%29+-%3E+Result%3Ci32%2C+ParseIntError%3E+%7B%0A++++let+processing_fee+%3D+1%3B%0A++++let+cost_per_item+%3D+5%3B%0A++++let+qty+%3D+item_quantity.parse%3A%3A%3Ci32%3E%28%29%3F%3B%0A%0A++++Ok%28qty+*+cost_per_item+%2B+processing_fee%29%0A%7D)
- ["errorsn.rs"](https://play.rust-lang.org/?code=%2F%2F+errorsn.rs%0A%0Ause+std%3A%3Aerror%3B%0Ause+std%3A%3Afmt%3B%0Ause+std%3A%3Aio%3B%0A%0A%2F%2F+PositiveNonzeroInteger+is+a+struct+defined+below+the+tests.%0Afn+read_and_validate%28b%3A+%26mut+io%3A%3ABufRead%29+-%3E+Result%3CPositiveNonzeroInteger%2C+Box%3Cerror%3A%3AError%3E%3E+%7B%0A++++let+mut+line+%3D+String%3A%3Anew%28%29%3B%0A++++b.read_line%28%26mut+line%29%3F%3B%0A++++let+num%3A+i64+%3D+line.trim%28%29.parse%28%29%3F%3B%0A++++let+answer+%3D+PositiveNonzeroInteger%3A%3Anew%28num%29%3F%3B%0A++++Ok%28answer%29%0A%7D%0A%0A%2F%2F+This+is+a+test+helper+function+that+turns+a+%26str+into+a+BufReader.%0Afn+test_with_str%28s%3A+%26str%29+-%3E+Result%3CPositiveNonzeroInteger%2C+Box%3Cerror%3A%3AError%3E%3E+%7B%0A++++let+mut+b+%3D+io%3A%3ABufReader%3A%3Anew%28s.as_bytes%28%29%29%3B%0A++++read_and_validate%28%26mut+b%29%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_success%28%29+%7B%0A++++let+x+%3D+test_with_str%28%2242%5Cn%22%29%3B%0A++++assert_eq%21%28PositiveNonzeroInteger%2842%29%2C+x.unwrap%28%29%29%3B%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_not_num%28%29+%7B%0A++++let+x+%3D+test_with_str%28%22eleven+billion%5Cn%22%29%3B%0A++++assert%21%28x.is_err%28%29%29%3B%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_non_positive%28%29+%7B%0A++++let+x+%3D+test_with_str%28%22-40%5Cn%22%29%3B%0A++++assert%21%28x.is_err%28%29%29%3B%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_ioerror%28%29+%7B%0A++++struct+Broken%3B%0A++++impl+io%3A%3ARead+for+Broken+%7B%0A++++++++fn+read%28%26mut+self%2C+_buf%3A+%26mut+%5Bu8%5D%29+-%3E+io%3A%3AResult%3Cusize%3E+%7B%0A++++++++++++Err%28io%3A%3AError%3A%3Anew%28io%3A%3AErrorKind%3A%3ABrokenPipe%2C+%22uh-oh%21%22%29%29%0A++++++++%7D%0A++++%7D%0A++++let+mut+b+%3D+io%3A%3ABufReader%3A%3Anew%28Broken%29%3B%0A++++assert%21%28read_and_validate%28%26mut+b%29.is_err%28%29%29%3B%0A++++assert_eq%21%28%22uh-oh%21%22%2C+read_and_validate%28%26mut+b%29.unwrap_err%28%29.to_string%28%29%29%3B%0A%7D%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Astruct+PositiveNonzeroInteger%28u64%29%3B%0A%0Aimpl+PositiveNonzeroInteger+%7B%0A++++fn+new%28value%3A+i64%29+-%3E+Result%3CPositiveNonzeroInteger%2C+CreationError%3E+%7B%0A++++++++if+value+%3D%3D+0+%7B%0A++++++++++++Err%28CreationError%3A%3AZero%29%0A++++++++%7D+else+if+value+%3C+0+%7B%0A++++++++++++Err%28CreationError%3A%3ANegative%29%0A++++++++%7D+else+%7B%0A++++++++++++Ok%28PositiveNonzeroInteger%28value+as+u64%29%29%0A++++++++%7D%0A++++%7D%0A%7D%0A%0A%23%5Btest%5D%0Afn+test_positive_nonzero_integer_creation%28%29+%7B%0A++++assert%21%28PositiveNonzeroInteger%3A%3Anew%2810%29.is_ok%28%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3ANegative%29%2C+PositiveNonzeroInteger%3A%3Anew%28-10%29%29%3B%0A++++assert_eq%21%28Err%28CreationError%3A%3AZero%29%2C+PositiveNonzeroInteger%3A%3Anew%280%29%29%3B%0A%7D%0A%0A%23%5Bderive%28PartialEq%2CDebug%29%5D%0Aenum+CreationError+%7B%0A++++Negative%2C%0A++++Zero%2C%0A%7D%0A%0Aimpl+fmt%3A%3ADisplay+for+CreationError+%7B%0A++++fn+fmt%28%26self%2C+f%3A+%26mut+fmt%3A%3AFormatter%29+-%3E+fmt%3A%3AResult+%7B%0A++++++++f.write_str%28%28self+as+%26error%3A%3AError%29.description%28%29%29%0A++++%7D%0A%7D%0A%0Aimpl+error%3A%3AError+for+CreationError+%7B%0A++++fn+description%28%26self%29+-%3E+%26str+%7B%0A++++++++match+*self+%7B%0A++++++++++++CreationError%3A%3ANegative+%3D%3E+%22Negative%22%2C%0A++++++++++++CreationError%3A%3AZero+%3D%3E+%22Zero%22%2C%0A++++++++%7D%0A++++%7D%0A%7D)

### Standard library types

#### `Arc`

The [Concurrency](https://doc.rust-lang.org/book/second-edition/ch16-03-shared-state.html) section
is relevant.

- ["arc1.rs"](https://play.rust-lang.org/?code=%2F%2F+arc1.rs%0A%0Ause+std%3A%3Async%3A%3AArc%3B%0Ause+std%3A%3Athread%3B%0A%0Afn+main%28%29+%7B%0A++++let+numbers%3A+Vec%3C_%3E+%3D+%280..100u32%29.collect%28%29%3B%0A++++let+shared_numbers+%3D+Arc%3A%3Anew%28numbers%29%3B%0A++++let+mut+joinhandles+%3D+Vec%3A%3Anew%28%29%3B%0A%0A++++for+offset+in+0..8+%7B%0A++++++++let+child_numbers+%3D+Arc%3A%3Aclone%28%26shared_numbers%29%3B%0A++++++++joinhandles.push%28%0A++++++++thread%3A%3Aspawn%28move+%7C%7C+%7B%0A++++++++++++let+mut+i+%3D+offset%3B%0A++++++++++++let+mut+sum+%3D+0%3B%0A++++++++++++while+i+%3C+child_numbers.len%28%29+%7B%0A++++++++++++++++sum+%2B%3D+child_numbers%5Bi%5D%3B%0A++++++++++++++++i+%2B%3D+5%3B%0A++++++++++++%7D%0A++++++++++++println%21%28%22Sum+of+offset+%7B%7D+is+%7B%7D%22%2C+offset%2C+sum%29%3B%0A++++++++%7D%29%29%3B%0A++++%7D%0A++++for+handle+in+joinhandles.into_iter%28%29+%7B%0A++++++++handle.join%28%29.unwrap%28%29%3B%0A++++%7D%0A%7D)

#### Iterators

Do not adjust your monitors-- iterators 1 and 2 are indeed missing. Iterator 3 is a bit challenging
so we're leaving space for some exercises to lead up to it!

Check out the [Iterators chapter of the
book](https://doc.rust-lang.org/book/second-edition/ch13-02-iterators.html) and the [Iterator
docs](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html).

- ["iterator3.rs"](https://play.rust-lang.org/?code=%2F%2F+iterator3.rs%0A%0A%23%5Bderive%28Debug%2C+PartialEq%2C+Eq%29%5D%0Apub+enum+DivisionError+%7B%0A++++NotDivisible%28NotDivisibleError%29%2C%0A++++DivideByZero%2C%0A%7D%0A%0A%23%5Bderive%28Debug%2C+PartialEq%2C+Eq%29%5D%0Apub+struct+NotDivisibleError+%7B%0A++++dividend%3A+i32%2C%0A++++divisor%3A+i32%2C%0A%7D%0A%0A%2F%2F+This+function+should+calculate+%60a%60+divided+by+%60b%60+if+%60a%60+is%0A%2F%2F+evenly+divisible+by+b.%0A%2F%2F+Otherwise%2C+it+should+return+a+suitable+error.%0Apub+fn+divide%28a%3A+i32%2C+b%3A+i32%29+-%3E+Result%3Ci32%2C+DivisionError%3E+%7B%0A++++if+b+%3D%3D+0+%7B%0A++++++++Err%28DivisionError%3A%3ADivideByZero%29%0A++++%7D+else+if+a+%25+b+%3D%3D+0+%7B%0A++++++++Ok%28a+%2F+b%29%0A++++%7D+else+%7B%0A++++++++Err%28DivisionError%3A%3ANotDivisible%28NotDivisibleError+%7Bdividend%3A+a%2C+divisor%3A+b%7D%29%29%0A++++%7D%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%2F%2F+Tests+that+verify+your+%60divide%60+function+implementation%0A++++%23%5Btest%5D%0A++++fn+test_success%28%29+%7B%0A++++++++assert_eq%21%28divide%2881%2C+9%29%2C+Ok%289%29%29%3B%0A++++%7D%0A%0A++++%23%5Btest%5D%0A++++fn+test_not_divisible%28%29+%7B%0A++++++++assert_eq%21%28%0A++++++++++++divide%2881%2C+6%29%2C%0A++++++++++++Err%28DivisionError%3A%3ANotDivisible%28NotDivisibleError%7B%0A++++++++++++++++dividend%3A+81%2C%0A++++++++++++++++divisor%3A+6%0A++++++++++++%7D%29%29%0A++++++++%29%3B%0A++++%7D%0A%0A++++%23%5Btest%5D%0A++++fn+test_divide_by_0%28%29+%7B%0A++++++++assert_eq%21%28divide%2881%2C+0%29%2C+Err%28DivisionError%3A%3ADivideByZero%29%29%3B%0A++++%7D%0A%0A++++%23%5Btest%5D%0A++++fn+test_divide_0_by_something%28%29+%7B%0A++++++++assert_eq%21%28divide%280%2C+81%29%2C+Ok%280%29%29%3B%0A++++%7D%0A%0A++++%2F%2F+Iterator+exercises+using+your+%60divide%60+function%0A++++%23%5Btest%5D%0A++++fn+result_with_list%28%29+%7B%0A++++++++let+numbers+%3D+vec%21%5B27%2C+297%2C+38502%2C+81%5D%3B%0A++++++++let+division_results+%3D+numbers.into_iter%28%29.map%28%7Cn%7C+divide%28n%2C+27%29%29%3B%0A++++++++%2F%2F+let+x%3A+Result%3CVec%3C_%3E%2C+_%3E+%3D+division_results.collect%28%29%3B%0A++++++++let+x+%3D+division_results.collect%3A%3A%3CResult%3CVec%3C_%3E%2C+_%3E%3E%28%29%3B%0A++++++++assert_eq%21%28format%21%28%22%7B%3A%3F%7D%22%2C+x%29%2C+%22Ok%28%5B1%2C+11%2C+1426%2C+3%5D%29%22%29%3B%0A++++%7D%0A%0A++++%23%5Btest%5D%0A++++fn+list_of_results%28%29+%7B%0A++++++++let+numbers+%3D+vec%21%5B27%2C+297%2C+38502%2C+81%5D%3B%0A++++++++let+division_results+%3D+numbers.into_iter%28%29.map%28%7Cn%7C+divide%28n%2C+27%29%29%3B%0A++++++++%2F%2F+let+x%3A+Vec%3C_%3E+%3D+division_results.collect%28%29%3B%0A++++++++let+x+%3D+division_results.collect%3A%3A%3CVec%3C_%3E%3E%28%29%3B%0A++++++++assert_eq%21%28format%21%28%22%7B%3A%3F%7D%22%2C+x%29%2C+%22%5BOk%281%29%2C+Ok%2811%29%2C+Ok%281426%29%2C+Ok%283%29%5D%22%29%3B%0A++++%7D%0A%7D)
- ["iterators4.rs"](https://play.rust-lang.org/?code=%2F%2F+iterators4.rs%0A%0Apub+fn+factorial%28num%3A+u64%29+-%3E+u64+%7B%0A++++%2F%2F+T1%0A++++%2F%2F+let+numbers%3A+Vec%3Cu64%3E+%3D+%281..%3Dnum%29.collect%28%29%3B%0A++++%2F%2F+numbers.iter%28%29.product%28%29%0A++++%2F%2F+T2%0A++++%2F%2F+%281..%29.take_while%28%7C%26i%7C+i+%3C%3D+num%29.product%28%29%0A++++%2F%2F+T3%0A++++%281..%3Dnum%29.product%28%29%0A%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod+tests+%7B%0A++++use+super%3A%3A*%3B%0A%0A++++%23%5Btest%5D%0A++++fn+factorial_of_1%28%29+%7B%0A++++++++assert_eq%21%281%2C+factorial%281%29%29%3B%0A++++%7D%0A++++%23%5Btest%5D%0A++++fn+factorial_of_2%28%29+%7B%0A++++++++assert_eq%21%282%2C+factorial%282%29%29%3B%0A++++%7D%0A%0A++++%23%5Btest%5D%0A++++fn+factorial_of_4%28%29+%7B%0A++++++++assert_eq%21%2824%2C+factorial%284%29%29%3B%0A++++%7D%0A%7D)

### Threads

See [the Dining Philosophers
example](https://doc.rust-lang.org/1.4.0/book/dining-philosophers.html) and the
[Concurrency Chapter](https://doc.rust-lang.org/book/second-edition/ch16-01-threads.html) from the
book.

- ["threads1.rs"](https://play.rust-lang.org/?code=%2F%2F+threads1.rs%0A%2F%2F+Make+this+compile%21+Scroll+down+for+hints+%3A%29+The+idea+is+the+thread%0A%2F%2F+spawned+on+line+19+is+completing+jobs+while+the+main+thread+is%0A%2F%2F+monitoring+progress+until+10+jobs+are+completed.+If+you+see+6+lines%0A%2F%2F+of+%22waiting...%22+and+the+program+ends+without+timing+out+the+playground%2C%0A%2F%2F+you%27ve+got+it+%3A%29%0A%0Ause+std%3A%3Async%3A%3A%7BArc%2CMutex%7D%3B%0Ause+std%3A%3Athread%3B%0Ause+std%3A%3Atime%3A%3ADuration%3B%0A%0Astruct+JobStatus+%7B%0A++++jobs_completed%3A+u32%2C%0A%7D%0A%0Afn+main%28%29+%7B%0A++++let+status+%3D+Arc%3A%3Anew%28Mutex%3A%3Anew%28JobStatus+%7B+jobs_completed%3A+0+%7D%29%29%3B%0A++++let+status_shared+%3D+status.clone%28%29%3B%0A++++thread%3A%3Aspawn%28move+%7C%7C+%7B%0A++++++++for+_+in+0..10+%7B%0A++++++++++++thread%3A%3Asleep%28Duration%3A%3Afrom_millis%28250%29%29%3B%0A++++++++++++let+mut+stt+%3D+status_shared.lock%28%29.unwrap%28%29%3B%0A++++++++++++stt.jobs_completed+%2B%3D+1%3B%0A++++++++%7D%0A++++%7D%29%3B%0A++++let+status_main+%3D+status.clone%28%29%3B%0A++++let+mut+status_jobs+%3D+%7B%0A++++++++let+stt+%3D+status_main.lock%28%29.unwrap%28%29%3B%0A++++++++stt.jobs_completed%0A++++%7D%3B%0A++++while+status_jobs+%3C+10+%7B%0A++++++++println%21%28%22waiting...+%22%29%3B%0A++++++++thread%3A%3Asleep%28Duration%3A%3Afrom_millis%28500%29%29%3B%0A++++++++status_jobs+%3D+%7B%0A++++++++++++let+stt+%3D+status_main.lock%28%29.unwrap%28%29%3B%0A++++++++++++stt.jobs_completed%0A++++++++%7D%3B%0A++++%7D%0A%7D%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+%60Arc%60+is+an+Atomic+Reference+Counted+pointer+that+allows+safe%2C+shared+access%0A%2F%2F+to+**immutable**+data.+But+we+want+to+*change*+the+number+of+%60jobs_completed%60%0A%2F%2F+so+we%27ll+need+to+also+use+another+type+that+will+only+allow+one+thread+to%0A%2F%2F+mutate+the+data+at+a+time.+Take+a+look+at+this+section+of+the+book%3A%0A%2F%2F+https%3A%2F%2Fdoc.rust-lang.org%2Fstable%2Fbook%2Fsecond-edition%2Fch16-03-shared-state.html%23atomic-reference-counting-with-arct%0A%2F%2F+and+keep+scrolling+if+you%27d+like+more+hints+%3A%29%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Do+you+now+have+an+%60Arc%60+%60Mutex%60+%60JobStatus%60+at+the+beginning+of+main%3F+Like%3A%0A%2F%2F+%60let+status+%3D+Arc%3A%3Anew%28Mutex%3A%3Anew%28JobStatus+%7B+jobs_completed%3A+0+%7D%29%29%3B%60%0A%2F%2F+Similar+to+the+code+in+the+example+in+the+book+that+happens+after+the+text%0A%2F%2F+that+says+%22We+can+use+Arc%3CT%3E+to+fix+this.%22.+If+not%2C+give+that+a+try%21+If+you%0A%2F%2F+do+and+would+like+more+hints%2C+keep+scrolling%21%21%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%0A%2F%2F+Make+sure+neither+of+your+threads+are+holding+onto+the+lock+of+the+mutex%0A%2F%2F+while+they+are+sleeping%2C+since+this+will+prevent+the+other+thread+from%0A%2F%2F+being+allowed+to+get+the+lock.+Locks+are+automatically+released+when%0A%2F%2F+they+go+out+of+scope.%0A%0A%2F%2F+Ok%2C+so%2C+real+talk%2C+this+was+actually+tricky+for+*me*+to+do+too.+And%0A%2F%2F+I+could+see+a+lot+of+different+problems+you+might+run+into%2C+so+at+this%0A%2F%2F+point+I%27m+not+sure+which+one+you%27ve+hit+%3A%29+Please+see+a+few+possible%0A%2F%2F+answers+on+https%3A%2F%2Fgithub.com%2Fcarols10cents%2Frustlings%2Fissues%2F3+--%0A%2F%2F+mine+is+a+little+more+complicated+because+I+decided+I+wanted+to+see%0A%2F%2F+the+number+of+jobs+currently+done+when+I+was+checking+the+status.%0A%0A%2F%2F+Please+open+an+issue+if+you%27re+still+running+into+a+problem+that%0A%2F%2F+these+hints+are+not+helping+you+with%2C+or+if+you%27ve+looked+at+the+sample%0A%2F%2F+answers+and+don%27t+understand+why+they+work+and+yours+doesn%27t.%0A%0A%2F%2F+If+you%27ve+learned+from+the+sample+solutions%2C+I+encourage+you+to+come%0A%2F%2F+back+to+this+exercise+and+try+it+again+in+a+few+days+to+reinforce%0A%2F%2F+what+you%27ve+learned+%3A%29%0A)

### Uncategorized

A few exercises based on things I've encountered or had trouble with getting used to.

- ["ex1.rs"](https://play.rust-lang.org/?code=%2F%2F+ex1.rs%0D%0A%0D%0Afn+main%28%29+%7B%0D%0A++++println%21%28%29%3B%0D%0A%7D)
- ["ex2.rs"](https://play.rust-lang.org/?code=%2F%2F+ex2.rs%0D%0A%0D%0Afn+something%28%29+-%3E+String+%7B%0D%0A++++String%3A%3Afrom%28%22hi%21%22%29%0D%0A%7D%0D%0A%0D%0Afn+main%28%29+%7B%0D%0A++++println%21%28%22%7B%7D%22%2C+something%28%29%29%3B%0D%0A%7D)
- ["ex3.rs"](https://play.rust-lang.org/?code=%2F%2F+ex3.rs%0D%0A%0D%0A%23%5Bderive%28Debug%29%5D%0D%0Astruct+Foo+%7B%0D%0A++++capacity%3A+i32%2C%0D%0A%7D%0D%0A%0D%0Afn+main%28%29+%7B%0D%0A++++println%21%28%22%7B%3A%3F%7D%22%2C+Foo+%7B+capacity%3A+3+%7D%29%3B%0D%0A%7D)
- ["ex4.rs"](https://play.rust-lang.org/?code=%2F%2F+ex4.rs%0D%0A%0D%0Afn+something%28%29+-%3E+Result%3Ci32%2C+std%3A%3Anum%3A%3AParseIntError%3E+%7B%0D%0A++++let+x%3Ai32+%3D+%223%22.parse%28%29%3F%3B%0D%0A++++Ok%28x+*+4%29%0D%0A%7D%0D%0A%0D%0Afn+main%28%29+%7B%0D%0A++++match+something%28%29+%7B%0D%0A++++++++Ok%28..%29+%3D%3E+println%21%28%22You+win%21%22%29%2C%0D%0A++++++++Err%28e%29+%3D%3E+println%21%28%22Oh+no+something+went+wrong%3A+%7B%7D%22%2C+e%29%2C%0D%0A++++%7D%0D%0A%7D)
- ["ex5.rs"](https://play.rust-lang.org/?code=%2F%2F+ex5.rs%0D%0A%0D%0Aenum+Reaction%3C%27a%3E+%7B%0D%0A++++Sad%28%26%27a+str%29%2C%0D%0A++++Happy%28%26%27a+str%29%2C%0D%0A%7D%0D%0A%0D%0Afn+express%28sentiment%3A+%26Reaction%29+%7B%0D%0A++++match+sentiment+%7B%0D%0A++++++++Reaction%3A%3ASad%28s%29+%3D%3E+println%21%28%22%3A%28+%7B%7D%22%2C+s%29%2C%0D%0A++++++++Reaction%3A%3AHappy%28s%29+%3D%3E+println%21%28%22%3A%29+%7B%7D%22%2C+s%29%2C%0D%0A++++%7D%0D%0A%7D%0D%0A%0D%0Afn+main+%28%29+%7B%0D%0A++++let+x+%3D+Reaction%3A%3AHappy%28%22It%27s+a+great+day+for+Rust%21%22%29%3B%0D%0A++++express%28%26x%29%3B%0D%0A++++express%28%26x%29%3B%0D%0A++++let+y+%3D+Reaction%3A%3ASad%28%22This+code+doesn%27t+compile+yet.%22%29%3B%0D%0A++++express%28%26y%29%3B%0D%0A%7D)
- ["ex6.rs"](https://play.rust-lang.org/?code=%2F%2F+ex6.rs%0D%0A%0D%0Afn+main%28%29+%7B%0D%0A++++let+robot_name+%3D+Some%28String%3A%3Afrom%28%22Bors%22%29%29%3B%0D%0A%0D%0A++++match+robot_name+%7B%0D%0A++++++++Some%28ref+name%29+%3D%3E+println%21%28%22Found+a+name%3A+%7B%7D%22%2C+name%29%2C%0D%0A++++++++None+%3D%3E+%28%29%2C%0D%0A++++%7D%0D%0A%0D%0A++++println%21%28%22robot_name+is%3A+%7B%3A%3F%7D%22%2C+robot_name%29%3B%0D%0A%7D)

## To help with this repo/TODO list

* File issues for problems or suggestions!
* Contribute more exercises! Anything that took you time to get used to, or that you had trouble
  with, or that deserves practice would be a good exercise!
* How could the process of doing these exercises work better? This is an open-ended question :) Are
  the playground links good enough? Are there ways that we could make going to the next exercise
  easier without forking the playground??
