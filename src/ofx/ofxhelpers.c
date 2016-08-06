#include "../../doc/ofxCore.h"
#include <stdarg.h>
#include <stdio.h>


extern void param_get_varargs(void *handle, void *args);
extern void param_set_varargs(void *handle, void *args);

// TODO: pointer on different constant in this helper ?
// To avoid allocating CStrings over and over

int c_test_host(void *host) {
    OfxHost *ofxHost = (OfxHost*)host;
    if(ofxHost->fetchSuite(ofxHost->host, "OfxPropertySuite", 1) == 0) {
        return 1;
    };
    return 0;  
}

OfxStatus param_set_value (void *handle, ...) {
    va_list vaargs;
    va_start(vaargs, handle);
    param_set_varargs(handle, &handle + sizeof(void*));
    va_end(vaargs);
    return kOfxStatOK;
}

OfxStatus param_get_value (void *handle, ...) {
    va_list vaargs;
    va_start(vaargs, handle);
    printf("entering C compiled code\n");
    // cast handle
    //OfxParameterStruct *pstruct = (OfxParameterStruct *)handle;
    //@int nb_params = pstruct->get_nb_param();
    //int nb_params = get_nb_param(handle); // from rust compiled code
    param_get_varargs(handle, vaargs);
    //pstruct->param_get_3_int(pstruct, 1, 2, 3);
    //for (; n; n--){
    //    i = va_arg(ap, int); // check if there is a function to iterate over the params.
                               // how is it implemented in C ? (C book should have the answer)
    //  set_param(handle, n, ap);
    //}
    va_end(vaargs);
    return kOfxStatOK;
}

OfxStatus param_get_value_at_time (void *handle, OfxTime time, ...) {
    va_list vaargs;
    va_start(vaargs, time);
    printf("entering C compiled code\n");
    // cast handle
    //OfxParameterStruct *pstruct = (OfxParameterStruct *)handle;
    //@int nb_params = pstruct->get_nb_param();
    //pstruct->param_get_3_int(pstruct, 1, 2, 3);
    //for (; n; n--){
    //    i = va_arg(ap, int);
    //}
    va_end(vaargs);
    return kOfxStatOK;
}


OfxStatus param_get_derivative(void *param_handle, OfxTime time, ...) {

    return kOfxStatOK;
}

OfxStatus param_get_integral(void *param_handle, OfxTime time1, OfxTime time2, ...) {

    return kOfxStatOK;
}


OfxStatus param_set_value_at_time(void *param_handle, OfxTime time, ...) {

    return kOfxStatOK;
}


