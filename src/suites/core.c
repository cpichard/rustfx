#include "../ofx/ofxCore.h"
#include <stdarg.h>
#include <stdio.h>

//
int c_test_host(void *host) {
    OfxHost *ofxHost = (OfxHost*)host;
    if(ofxHost->fetchSuite(ofxHost->host, "OfxPropertySuite", 1) == 0) {
        return 1;
    };
    return 0;  
}
