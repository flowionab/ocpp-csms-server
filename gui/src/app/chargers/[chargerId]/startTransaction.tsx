"use server"

import {apiClient} from "@/app/api/grpc";

export async function startTransaction(chargerId: string, evseId: string) {
    await new Promise<void>((resolve, reject) => {
        apiClient.startTransaction({chargerId, evseId}, (error) => {
            if (error) {
                reject(error)
            } else {
                resolve()
            }
        })
    })
}