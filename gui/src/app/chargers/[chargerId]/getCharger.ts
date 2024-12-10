"use server"

import {GetChargerResponse} from "@/api_service";
import {apiClient} from "@/app/api/grpc";

export async function getCharger(chargerId: string): Promise<GetChargerResponse["charger"]> {
    const chargerData = await new Promise<GetChargerResponse>((resolve, reject) => {
        apiClient.getCharger({chargerId}, (error, response) => {
            if (error) {
                reject(error)
            } else {
                resolve(response)
            }
        })
    })

    return chargerData.charger;
}