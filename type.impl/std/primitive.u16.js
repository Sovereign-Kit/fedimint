(function() {var type_impls = {
"fedimint_core":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decodable-for-u16\" class=\"impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#298\">source</a><a href=\"#impl-Decodable-for-u16\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"fedimint_core/encoding/trait.Decodable.html\" title=\"trait fedimint_core::encoding::Decodable\">Decodable</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.consensus_decode\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#298\">source</a><a href=\"#method.consensus_decode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fedimint_core/encoding/trait.Decodable.html#method.consensus_decode\" class=\"fn\">consensus_decode</a>&lt;D: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt;(\n    d: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut D</a>,\n    _modules: &amp;<a class=\"type\" href=\"fedimint_core/module/registry/type.ModuleDecoderRegistry.html\" title=\"type fedimint_core::module::registry::ModuleDecoderRegistry\">ModuleDecoderRegistry</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, <a class=\"struct\" href=\"fedimint_core/encoding/struct.DecodeError.html\" title=\"struct fedimint_core::encoding::DecodeError\">DecodeError</a>&gt;</h4></section></summary><div class='docblock'>Decode an object with a well-defined format. <a href=\"fedimint_core/encoding/trait.Decodable.html#method.consensus_decode\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.consensus_decode_from_finite_reader\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#157-166\">source</a><a href=\"#method.consensus_decode_from_finite_reader\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fedimint_core/encoding/trait.Decodable.html#method.consensus_decode_from_finite_reader\" class=\"fn\">consensus_decode_from_finite_reader</a>&lt;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>&gt;(\n    r: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut R</a>,\n    modules: &amp;<a class=\"type\" href=\"fedimint_core/module/registry/type.ModuleDecoderRegistry.html\" title=\"type fedimint_core::module::registry::ModuleDecoderRegistry\">ModuleDecoderRegistry</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, <a class=\"struct\" href=\"fedimint_core/encoding/struct.DecodeError.html\" title=\"struct fedimint_core::encoding::DecodeError\">DecodeError</a>&gt;</h4></section></summary><div class='docblock'>Decode <code>Self</code> from a size-limited reader. <a href=\"fedimint_core/encoding/trait.Decodable.html#method.consensus_decode_from_finite_reader\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.consensus_decode_hex\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#186-195\">source</a><a href=\"#method.consensus_decode_hex\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fedimint_core/encoding/trait.Decodable.html#method.consensus_decode_hex\" class=\"fn\">consensus_decode_hex</a>(\n    hex: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>,\n    modules: &amp;<a class=\"type\" href=\"fedimint_core/module/registry/type.ModuleDecoderRegistry.html\" title=\"type fedimint_core::module::registry::ModuleDecoderRegistry\">ModuleDecoderRegistry</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, <a class=\"struct\" href=\"fedimint_core/encoding/struct.DecodeError.html\" title=\"struct fedimint_core::encoding::DecodeError\">DecodeError</a>&gt;</h4></section></summary><div class='docblock'>Decode an object from hex</div></details><section id=\"method.consensus_decode_vec\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#197-203\">source</a><a href=\"#method.consensus_decode_vec\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fedimint_core/encoding/trait.Decodable.html#method.consensus_decode_vec\" class=\"fn\">consensus_decode_vec</a>(\n    bytes: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;,\n    modules: &amp;<a class=\"type\" href=\"fedimint_core/module/registry/type.ModuleDecoderRegistry.html\" title=\"type fedimint_core::module::registry::ModuleDecoderRegistry\">ModuleDecoderRegistry</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, <a class=\"struct\" href=\"fedimint_core/encoding/struct.DecodeError.html\" title=\"struct fedimint_core::encoding::DecodeError\">DecodeError</a>&gt;</h4></section></div></details>","Decodable","fedimint_core::core::ModuleInstanceId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encodable-for-u16\" class=\"impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#298\">source</a><a href=\"#impl-Encodable-for-u16\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"fedimint_core/encoding/trait.Encodable.html\" title=\"trait fedimint_core::encoding::Encodable\">Encodable</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.consensus_encode\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#298\">source</a><a href=\"#method.consensus_encode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fedimint_core/encoding/trait.Encodable.html#tymethod.consensus_encode\" class=\"fn\">consensus_encode</a>&lt;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>&gt;(&amp;self, writer: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut W</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Encode an object with a well-defined format.\nReturns the number of bytes written on success. <a href=\"fedimint_core/encoding/trait.Encodable.html#tymethod.consensus_encode\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.consensus_encode_to_vec\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#76-81\">source</a><a href=\"#method.consensus_encode_to_vec\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fedimint_core/encoding/trait.Encodable.html#method.consensus_encode_to_vec\" class=\"fn\">consensus_encode_to_vec</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"Vec&lt;u8&gt;\">ⓘ</a></h4></section></summary><div class='docblock'><a href=\"fedimint_core/encoding/trait.Encodable.html#tymethod.consensus_encode\"><code>Self::consensus_encode</code></a> to newly allocated <code>Vec&lt;u8&gt;</code></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.consensus_encode_to_hex\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#84-92\">source</a><a href=\"#method.consensus_encode_to_hex\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fedimint_core/encoding/trait.Encodable.html#method.consensus_encode_to_hex\" class=\"fn\">consensus_encode_to_hex</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a></h4></section></summary><div class='docblock'>Encode and convert to hex string representation</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.consensus_encode_to_len\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#95-98\">source</a><a href=\"#method.consensus_encode_to_len\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fedimint_core/encoding/trait.Encodable.html#method.consensus_encode_to_len\" class=\"fn\">consensus_encode_to_len</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section></summary><div class='docblock'>Encode without storing the encoding, return the size</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.consensus_hash\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fedimint_core/encoding/mod.rs.html#104-113\">source</a><a href=\"#method.consensus_hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fedimint_core/encoding/trait.Encodable.html#method.consensus_hash\" class=\"fn\">consensus_hash</a>&lt;H&gt;(&amp;self) -&gt; H<div class=\"where\">where\n    H: <a class=\"trait\" href=\"fedimint_core/trait.BitcoinHash.html\" title=\"trait fedimint_core::BitcoinHash\">Hash</a>,\n    H::<a class=\"associatedtype\" href=\"fedimint_core/trait.BitcoinHash.html#associatedtype.Engine\" title=\"type fedimint_core::BitcoinHash::Engine\">Engine</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,</div></h4></section></summary><div class='docblock'>Generate a SHA256 hash of the consensus encoding using the default hash\nengine for <code>H</code>. <a href=\"fedimint_core/encoding/trait.Encodable.html#method.consensus_hash\">Read more</a></div></details></div></details>","Encodable","fedimint_core::core::ModuleInstanceId"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CPeerId%3E-for-u16\" class=\"impl\"><a class=\"src rightside\" href=\"src/fedimint_core/lib.rs.html#347-351\">source</a><a href=\"#impl-From%3CPeerId%3E-for-u16\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"fedimint_core/struct.PeerId.html\" title=\"struct fedimint_core::PeerId\">PeerId</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fedimint_core/lib.rs.html#348-350\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(peer: <a class=\"struct\" href=\"fedimint_core/struct.PeerId.html\" title=\"struct fedimint_core::PeerId\">PeerId</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a></h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<PeerId>","fedimint_core::core::ModuleInstanceId"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()