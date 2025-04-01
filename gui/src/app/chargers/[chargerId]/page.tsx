import {getCharger} from "@/app/chargers/[chargerId]/getCharger";
import StartTransactionButton from "@/app/chargers/[chargerId]/StartTransactionButton";
import ChangeOutletAvailabilityButton from "@/app/chargers/[chargerId]/ChangeOutletAvailabilityButton";

export const dynamic = 'force-dynamic'

export default async function Page(props: {
    params: Promise<{ chargerId: string }>
}) {
    const params = await props.params;
    const chargerId = params.chargerId;
    const charger = await getCharger(chargerId);


    if (!charger) {
        return <div>Not found - {chargerId}</div>
    }

    return <div className={"container mx-auto"}>
        <div className={"flex flex-row"}>
            <div>
                <h1 className={"text-5xl mt-8"}>{chargerId}</h1>
                <h2 className={"text-2xl text-neutral-400"}>{charger.serialNumber}</h2>
            </div>
            <div className={"flex flex-col justify-center ml-4"}>
                {
                    charger.isOnline ?
                        <div className={"h-4 w-4 rounded-full bg-green-500 green-dot-animation-pulse"}/> :
                        <div className={"h-4 w-4 rounded-full bg-red-500"}/>
                }
            </div>
        </div>

        <h2 className={"text-2xl mt-4"}>{charger.status}</h2>

        <h3 className={"text-3xl mt-8 mb-4"}>Outlets</h3>
        {
            charger.evses.map(evse => (
                <div key={evse.id} className={"rounded-lg dark:bg-neutral-800 shadow-md p-4 mb-4 flex flex-col"}>
                    <span className={"text-xl"}>Outlet {evse.ocppConnectorId} - {evse.status ?? "Unknown"}</span>
                    <StartTransactionButton chargerId={chargerId} outletId={evse.id}/>
                    <ChangeOutletAvailabilityButton chargerId={chargerId} outletId={evse.id} status={evse.status}/>
                    <span className={"text-xs dark:text-gray-400"}>{evse.id}</span>
                </div>))
        }
    </div>
}