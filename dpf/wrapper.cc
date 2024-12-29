#include "wrapper.h"
#include "distributed_point_function.h"
#include "../pir/dense_dpf_pir_client.h"
#include "../pir/dense_dpf_pir_server.h"
#include "../pir/dense_dpf_pir_database.h"

using namespace distributed_point_functions;

// Wrapper structs to hold C++ objects
struct DpfDatabase {
    std::unique_ptr<PirDatabaseInterface<XorWrapper<absl::uint128>, std::string>> db;
    std::vector<std::string> elements;
    int num_elements;
    int element_size;
};

struct DpfClient {
    std::unique_ptr<DenseDpfPirClient> client;
    PirConfig config;
};

struct DpfServer {
    std::unique_ptr<DenseDpfPirServer> server;
};

struct DpfRequest {
    PirRequest request;
    PirRequestClientState client_state;
};

struct DpfResponse {
    PirResponse response;
};

// Implementation of database operations
DpfStatus DpfCreateDatabase(int num_elements, int element_size_bytes, DpfDatabaseHandle* db) {
    try {
        auto database = new DpfDatabase();
        database->num_elements = num_elements;
        database->element_size = element_size_bytes;
        database->elements.resize(num_elements);
        
        // Initialize database with empty elements
        for (int i = 0; i < num_elements; i++) {
            database->elements[i] = std::string(element_size_bytes, '\0');
        }
        
        // Create database using builder pattern
        DenseDpfPirDatabase::Builder builder;
        for (const auto& element : database->elements) {
            builder.Insert(element);
        }
        
        auto status_or_db = builder.Build();
        if (!status_or_db.ok()) {
            delete database;
            return DPF_ERROR_INTERNAL;
        }
        
        database->db = std::move(status_or_db.value());
        *db = database;
        return DPF_SUCCESS;
    } catch (const std::bad_alloc&) {
        return DPF_ERROR_OUT_OF_MEMORY;
    } catch (...) {
        return DPF_ERROR_INTERNAL;
    }
}

DpfStatus DpfDestroyDatabase(DpfDatabaseHandle db) {
    if (!db) return DPF_ERROR_INVALID_ARGUMENT;
    delete db;
    return DPF_SUCCESS;
}

DpfStatus DpfDatabaseSetElement(DpfDatabaseHandle db, int index, const char* data, int data_size) {
    if (!db || !data || index < 0 || index >= db->num_elements || 
        data_size != db->element_size) {
        return DPF_ERROR_INVALID_ARGUMENT;
    }
    
    try {
        db->elements[index] = std::string(data, data_size);
        
        // Rebuild database with updated elements
        DenseDpfPirDatabase::Builder builder;
        for (const auto& element : db->elements) {
            builder.Insert(element);
        }
        
        auto status_or_db = builder.Build();
        if (!status_or_db.ok()) {
            return DPF_ERROR_INTERNAL;
        }
        db->db = std::move(status_or_db.value());
        return DPF_SUCCESS;
    } catch (...) {
        return DPF_ERROR_INTERNAL;
    }
}

// Implementation of client operations
DpfStatus DpfCreateClient(DpfClientHandle* client) {
    try {
        auto c = new DpfClient();
        
        // Initialize client config
        c->config.mutable_dense_dpf_pir_config();
        
        // Create client instance with null encrypter since we're using plain mode
        auto status_or_client = DenseDpfPirClient::Create(c->config, nullptr);
        if (!status_or_client.ok()) {
            delete c;
            return DPF_ERROR_INTERNAL;
        }
        
        c->client = std::move(status_or_client.value());
        *client = c;
        return DPF_SUCCESS;
    } catch (const std::bad_alloc&) {
        return DPF_ERROR_OUT_OF_MEMORY;
    } catch (...) {
        return DPF_ERROR_INTERNAL;
    }
}

DpfStatus DpfDestroyClient(DpfClientHandle client) {
    if (!client) return DPF_ERROR_INVALID_ARGUMENT;
    delete client;
    return DPF_SUCCESS;
}

// Implementation of server operations
DpfStatus DpfCreateServer(DpfDatabaseHandle db, DpfServerHandle* server) {
    if (!db || !server) return DPF_ERROR_INVALID_ARGUMENT;
    
    try {
        auto s = new DpfServer();
        
        // Create server config
        PirConfig config;
        auto* dpf_config = config.mutable_dense_dpf_pir_config();
        dpf_config->set_num_elements(db->num_elements);
        
        // Create server instance
        auto status_or_server = DenseDpfPirServer::CreatePlain(config, std::move(db->db));
        if (!status_or_server.ok()) {
            delete s;
            return DPF_ERROR_INTERNAL;
        }
        
        s->server = std::move(status_or_server.value());
        *server = s;
        return DPF_SUCCESS;
    } catch (const std::bad_alloc&) {
        return DPF_ERROR_OUT_OF_MEMORY;
    } catch (...) {
        return DPF_ERROR_INTERNAL;
    }
}

DpfStatus DpfDestroyServer(DpfServerHandle server) {
    if (!server) return DPF_ERROR_INVALID_ARGUMENT;
    delete server;
    return DPF_SUCCESS;
}

// Implementation of request/response operations
DpfStatus DpfCreateRequest(DpfClientHandle client, int index, DpfRequestHandle* request) {
    if (!client || !request || index < 0) return DPF_ERROR_INVALID_ARGUMENT;
    
    try {
        auto req = new DpfRequest();
        
        // Create vector with single index
        std::vector<int> indices = {index};
        
        // Generate PIR request for the given index
        auto status_or_request = client->client->CreateRequest(absl::MakeConstSpan(indices));
        if (!status_or_request.ok()) {
            delete req;
            return DPF_ERROR_INTERNAL;
        }
        
        auto [request_proto, client_state] = std::move(status_or_request.value());
        req->request = std::move(request_proto);
        req->client_state = std::move(client_state);
        *request = req;
        return DPF_SUCCESS;
    } catch (const std::bad_alloc&) {
        return DPF_ERROR_OUT_OF_MEMORY;
    } catch (...) {
        return DPF_ERROR_INTERNAL;
    }
}

DpfStatus DpfDestroyRequest(DpfRequestHandle request) {
    if (!request) return DPF_ERROR_INVALID_ARGUMENT;
    delete request;
    return DPF_SUCCESS;
}

DpfStatus DpfProcessRequest(DpfServerHandle server, DpfRequestHandle request, DpfResponseHandle* response) {
    if (!server || !request || !response) return DPF_ERROR_INVALID_ARGUMENT;
    
    try {
        auto resp = new DpfResponse();
        
        // Process the PIR request
        auto status_or_response = server->server->HandleRequest(request->request);
        if (!status_or_response.ok()) {
            delete resp;
            return DPF_ERROR_INTERNAL;
        }
        
        resp->response = std::move(status_or_response.value());
        *response = resp;
        return DPF_SUCCESS;
    } catch (const std::bad_alloc&) {
        return DPF_ERROR_OUT_OF_MEMORY;
    } catch (...) {
        return DPF_ERROR_INTERNAL;
    }
}

DpfStatus DpfDestroyResponse(DpfResponseHandle response) {
    if (!response) return DPF_ERROR_INVALID_ARGUMENT;
    delete response;
    return DPF_SUCCESS;
}

DpfStatus DpfGetResult(DpfClientHandle client, DpfRequestHandle request, DpfResponseHandle response, char** result, int* result_size) {
    if (!client || !request || !response || !result || !result_size) return DPF_ERROR_INVALID_ARGUMENT;
    
    try {
        // Get the result from the response
        auto status_or_result = client->client->HandleResponse(response->response, request->client_state);
        if (!status_or_result.ok()) {
            return DPF_ERROR_INTERNAL;
        }
        
        // We expect a single result since we only query one index
        auto results = std::move(status_or_result.value());
        if (results.empty()) {
            return DPF_ERROR_INTERNAL;
        }
        
        std::string& result_str = results[0];
        *result_size = result_str.size();
        
        // Allocate memory for the result
        *result = new char[*result_size];
        std::memcpy(*result, result_str.data(), *result_size);
        
        return DPF_SUCCESS;
    } catch (const std::bad_alloc&) {
        return DPF_ERROR_OUT_OF_MEMORY;
    } catch (...) {
        return DPF_ERROR_INTERNAL;
    }
}

DpfStatus DpfDestroyResult(char* result) {
    if (!result) return DPF_ERROR_INVALID_ARGUMENT;
    delete[] result;
    return DPF_SUCCESS;
}