"use server"

import {apiClient} from "@/app/api/grpc";

export async function changeEvseAvailability(chargerId: string, evseId: string, operative: boolean) {
    await new Promise<void>((resolve, reject) => {
        apiClient.changeEvseAvailability({chargerId, evseId, operative}, (error) => {
            if (error) {
                reject(error)
            } else {
                resolve()
            }
        })
    })
}