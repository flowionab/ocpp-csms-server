"use server"

import {apiClient} from "@/app/api/grpc";

export async function changeOutletAvailability(chargerId: string, outletId: string, available: boolean) {
    await new Promise<void>((resolve, reject) => {
        apiClient.changeOutletAvailability({chargerId, outletId, available}, (error) => {
            if (error) {
                reject(error)
            } else {
                resolve()
            }
        })
    })
}