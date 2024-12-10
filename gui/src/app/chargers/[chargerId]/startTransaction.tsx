"use server"

import {apiClient} from "@/app/api/grpc";

export async function startTransaction(chargerId: string, outletId: string) {
    await new Promise<void>((resolve, reject) => {
        apiClient.startTransaction({chargerId, outletId}, (error) => {
            if (error) {
                reject(error)
            } else {
                resolve()
            }
        })
    })
}