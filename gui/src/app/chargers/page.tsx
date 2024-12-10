import {GetChargersResponse} from "@/api_service";
import {apiClient} from "@/app/api/grpc";
import Link from "next/link";

export const dynamic = 'force-dynamic'

export default async function Page() {

    const chargersData = await new Promise<GetChargersResponse>((resolve, reject) => {
        apiClient.getChargers({page: 0, pageSize: 50}, (error, response) => {
            if (error) {
                reject(error)
            } else {
                resolve(response)
            }
        })
    })

    const chargers = chargersData.chargers;
    return <div className={"flex-1 flex flex-col container mx-auto"}>
        <h1 className={"text-5xl mt-4"}>Chargers</h1>
        <h1 className={"text mt-2 text-gray-400"}>Here is a list of all your chargers</h1>
        <div className={"grid grid-cols-4 gap-4"}>
            {chargers.map(i => (
                <Link href={`/chargers/${i.id}`} className={"border flex flex-col p-2 rounded-md h-48"} key={i.id}>
                    <span className={"text-xl"}>{i.serialNumber}</span>
                    <div className={"flex-1"}/>
                    <span className={"text-md"}>{i.vendor}</span>
                    <span className={"text-md"}>{i.model}</span>
                    <span className={"text-md"}>{i.firmwareVersion}</span>
                    <span className={"text-xs text-gray-300 mt-1"}>{i.id}</span>
                </Link>))}
        </div>
    </div>
}