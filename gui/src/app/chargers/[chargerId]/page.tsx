import {apiClient} from "@/app/api/grpc";
import {GetChargerResponse} from "@/api_service";

export const dynamic = 'force-dynamic'

export default async function Page(props: any) {
    const params = await props.params;
    const chargerId = params.chargerId;

    const chargerData = await new Promise<GetChargerResponse>((resolve, reject) => {
        apiClient.getCharger({chargerId}, (error, response) => {
            if (error) {
                reject(error)
            } else {
                resolve(response)
            }
        })
    })

    const charger = chargerData.charger;

    if (!charger) {
        return <div>Not found - {chargerId}</div>
    }

    return <div>
        <h1 className={"text-5xl mt-4"}>Hello {charger.serialNumber ?? chargerId}</h1>
        <span className={"text-sm text-gray-300"}>{chargerId}</span>
        <span className={"text-sm text-gray-300"}>{JSON.stringify(chargerData)}</span>
    </div>
}