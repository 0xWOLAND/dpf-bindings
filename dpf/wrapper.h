#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_WRAPPER_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_WRAPPER_H_

#ifdef __cplusplus
extern "C" {
#endif

// Opaque types to represent the C++ objects
typedef struct DpfDatabase* DpfDatabaseHandle;
typedef struct DpfClient* DpfClientHandle;
typedef struct DpfServer* DpfServerHandle;
typedef struct DpfRequest* DpfRequestHandle;
typedef struct DpfResponse* DpfResponseHandle;

// Error codes
typedef enum {
    DPF_SUCCESS = 0,
    DPF_ERROR_INVALID_ARGUMENT = 1,
    DPF_ERROR_INTERNAL = 2,
    DPF_ERROR_OUT_OF_MEMORY = 3
} DpfStatus;

// Database operations
DpfStatus DpfCreateDatabase(int num_elements, int element_size_bytes, DpfDatabaseHandle* db);
DpfStatus DpfDestroyDatabase(DpfDatabaseHandle db);
DpfStatus DpfDatabaseSetElement(DpfDatabaseHandle db, int index, const char* data, int data_size);

// Client operations
DpfStatus DpfCreateClient(DpfClientHandle* client);
DpfStatus DpfDestroyClient(DpfClientHandle client);

// Server operations
DpfStatus DpfCreateServer(DpfDatabaseHandle db, DpfServerHandle* server);
DpfStatus DpfDestroyServer(DpfServerHandle server);

// Request/Response operations
DpfStatus DpfCreateRequest(DpfClientHandle client, int index, DpfRequestHandle* request);
DpfStatus DpfDestroyRequest(DpfRequestHandle request);
DpfStatus DpfProcessRequest(DpfServerHandle server, DpfRequestHandle request, DpfResponseHandle* response);
DpfStatus DpfDestroyResponse(DpfResponseHandle response);
DpfStatus DpfGetResult(DpfClientHandle client, DpfRequestHandle request, DpfResponseHandle response, char** result, int* result_size);
DpfStatus DpfDestroyResult(char* result);

#ifdef __cplusplus
}
#endif

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_WRAPPER_H_
