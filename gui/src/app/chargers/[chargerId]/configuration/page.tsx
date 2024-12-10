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

    return <div className={"container mx-auto"}>
        <h1 className={"text-5xl my-8"}>OCPP Settings</h1>
        <div className={"grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 my-4"}>
            {
                charger.ocpp16ConfigurationValues.map((i) => (
                    <div key={i.key} className={"p-4 border h-48 dark:bg-neutral-800 border-neutral-900 flex flex-col"}>
                        <div className={"flex-1 flex flex-col justify-center"}>
                            {i.value?.split(",").map((j) => (<span
                                className={`text-center ${(i.value?.split(",").length ?? 1) > 3 ? "text-xs" : "text-xl"} ${mapValueColor(j)}`}>{mapValue(j)}</span>))}

                        </div>
                        <div className={"mx-8 flex flex-col"}>
                            <span className={"text-sm text-gray-500"}>{i.readonly ? "Read" : "Read/Write"}</span>
                            <span
                                className={i.key.length > 30 ? i.key.length > 40 ? "text-xs" : "text-sm" : "text-xl"}>{i.key}</span>
                        </div>
                    </div>))
            }
        </div>


    </div>
}

function mapValue(val: string | undefined) {
    if (val === "true") {
        return "True"
    }
    if (val === "false") {
        return "False"
    }
    if (val === "") {
        return "Null"
    }
    if (val === undefined) {
        return "Null"
    }
    return val
}

function mapValueColor(val: string | undefined) {
    if (val === "true") {
        return "text-green-400"
    }
    if (val === "false") {
        return "text-red-400"
    }
    if (val === "") {
        return "text-gray-500"
    }
    if (val === undefined) {
        return "text-gray-500"
    }
    return val
}