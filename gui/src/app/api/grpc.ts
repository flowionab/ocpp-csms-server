import {ApiClient} from "@/api_service";
import {credentials} from "@grpc/grpc-js";


export const apiClient = new ApiClient('localhost:50052', credentials.createInsecure());
