#include "../ofx/ofxCore.h"
#include <stdarg.h>
#include <stdio.h>

// TODO: handle message suite
int c_message(void *handle, const char *messageType, const char * messageId, const char * format, ...)
{
    return kOfxStatOK;
}

// TODO: handle message suite
int c_set_persistent_message(void *handle, const char *messageType, const char * messageId, const char * format, ...)
{
    return kOfxStatOK;
}
