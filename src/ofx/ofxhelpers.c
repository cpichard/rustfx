#include "../../doc/ofxCore.h"
#include <stdarg.h>
#include <stdio.h>


// Rust callbacks
extern unsigned int param_get_nb_component(void *handle);
extern void param_set_components(void *handle, void *data);
extern void param_get_components(void *handle, void *data);
extern unsigned int param_get_type(void *handle);

// TODO: pointer on different constant in this helper ?
// To avoid allocating CStrings over and over

int c_test_host(void *host) {
    OfxHost *ofxHost = (OfxHost*)host;
    if(ofxHost->fetchSuite(ofxHost->host, "OfxPropertySuite", 1) == 0) {
        return 1;
    };
    return 0;  
}

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

OfxStatus param_set_value (void *handle, ...) {
    va_list vaargs;
    va_start(vaargs, handle);
    
    const unsigned int nb = param_get_nb_component(handle);
    const unsigned int tp = param_get_type(handle); // replace by param_type_is_float/int/string
    if (tp==0) {
        int data[nb];
        for (unsigned int i=0; i < nb; i++)
        {
            data[i] = va_arg(vaargs, int);
        }
        param_set_components(handle, &data[0]);
    } else if (tp==1) {
        double data[nb];
        for (unsigned int i=0; i < nb; i++)
        {
            data[i] = va_arg(vaargs, double);
        }
        param_set_components(handle, &data[0]);
    } else {
        // error
        printf("error, parameter type unknown\n");
    }

    va_end(vaargs);
    return kOfxStatOK;
}

OfxStatus param_get_value (void *handle, ...) {
    va_list vaargs;
    va_start(vaargs, handle);
    
    const unsigned int nb = param_get_nb_component(handle);
    const unsigned int tp = param_get_type(handle); // replace by param_type_is_float/int/string
    if (tp==0) {
        int data[nb];
        param_get_components(handle, &data[0]);
        for (unsigned int i=0; i < nb; i++)
        {
            int *val = va_arg(vaargs, int*); 
            *val = data[i];
        }
    } else if (tp==1) {
        double data[nb];
        param_get_components(handle, &data[0]);
        for (unsigned int i=0; i < nb; i++)
        {
            double * val = va_arg(vaargs, double*); 
            *val = data[i];
        }
        param_set_components(handle, &data[0]);
    } else {
        // error
        printf("error, parameter type unknown\n");
    }

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


