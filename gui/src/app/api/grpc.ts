import {ApiClient} from "@/api_service";
import {credentials} from "@grpc/grpc-js";


export const apiClient = new ApiClient(process.env.API_URL || 'localhost:50053', credentials.createInsecure());
