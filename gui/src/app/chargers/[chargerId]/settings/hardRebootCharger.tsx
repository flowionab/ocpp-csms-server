"use server"

import {apiClient} from "@/app/api/grpc";
import {RebootChargerRequest_RebootType} from "@/reboot_charger";

export async function hardRebootCharger(chargerId: string) {
    await new Promise<void>((resolve, reject) => {
        apiClient.rebootCharger({chargerId, rebootType: RebootChargerRequest_RebootType.Hard}, (error) => {
            if (error) {
                reject(error)
            } else {
                resolve()
            }
        })
    })
}