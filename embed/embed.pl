use FFI::Platypus;
my $ffi = FFI::Platypus->new;
$ffi->lang('Rust');
$ffi->lib('target/release/libembed.dylib');

$ffi->attach('process', ['void'] => 'void');

process();

print "done"
