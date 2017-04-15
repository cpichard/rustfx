#include "../include/ofxCore.h"
#include <stdarg.h>
#include <stdio.h>


// Rust callbacks
extern unsigned int param_get_nb_component(void *handle);
extern void param_set_components(void *handle, void *data);
extern void param_get_components(void *handle, void *data);
extern unsigned int param_get_type(void *handle);

// TODO: pointer on different constant in this helper ?
// To avoid allocating CStrings over and over

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
        // TODO: return correct error code and get rid of printf
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
        //
        // TODO return corresponding ofx error
        //  va_end before returning
        printf("ERROR:param.c parameter type unknown\n");
    }

    va_end(vaargs);
    return kOfxStatOK;
}

OfxStatus param_get_value_at_time (void *handle, OfxTime time, ...) {
    va_list vaargs;
    va_start(vaargs, time);
    printf("ERROR:param.c param_get_value_at_time not implemented\n");
    va_end(vaargs);
    return kOfxStatOK;
}


OfxStatus param_get_derivative(void *param_handle, OfxTime time, ...) {

    // TODO: implement get_derivative
    printf("ERROR:param.c param_get_derivative not implemented\n");

    return kOfxStatOK;
}

OfxStatus param_get_integral(void *param_handle, OfxTime time1, OfxTime time2, ...) {

    // TODO implement get integral

    return kOfxStatOK;
}


OfxStatus param_set_value_at_time(void *param_handle, OfxTime time, ...) {

    // TODO
    printf("ERROR:param.c param_set_value_at_time not implemented\n");
    //
    return kOfxStatOK;
}


