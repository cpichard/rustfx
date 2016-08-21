#!/usr/bin/awk -f
{
    if ( match($3, "\"[a-zA-Z0-9]+\"") ) {
        pattern = substr($3, RSTART+1, RLENGTH-2);
        print "pub const "$2": &'static [u8] = b\""pattern"\\0\";"
    } else if (match ($3, "0x[a-zA-Z0-9]+")) {
        pattern = substr($3,RSTART,RLENGTH);
        print "pub const "$2": u32 = "pattern";"
    } else if (match ($3, "[0-9]+")) {
        int_value = substr($3, RSTART, RLENGTH);
        print "pub const "$2": OfxStatus = "int_value";"
    } else if (match ($4, "[0-9]+")) {
        int_value = substr($4, RSTART, RLENGTH);
        print "pub const "$2": OfxStatus = "int_value";"
    } else {
        print "// unrecognized", $0
    }
}
