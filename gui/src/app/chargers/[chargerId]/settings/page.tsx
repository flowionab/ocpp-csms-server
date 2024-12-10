import {getCharger} from "@/app/chargers/[chargerId]/getCharger";
import HardRebootButton from "@/app/chargers/[chargerId]/settings/HardRebootButton";
import SoftRebootButton from "@/app/chargers/[chargerId]/settings/SoftRebootButton";

export default async function Page(props: any) {
    const params = await props.params;
    const chargerId = params.chargerId;
    const charger = await getCharger(chargerId);

    return <div className={"flex-1 flex flex-col container mx-auto"}>
        <h1 className={"text-3xl mt-4"}>{charger?.id}</h1>
        <h2 className={"text-xl"}>{charger?.serialNumber}</h2>

        <div className={"mt-8 flex flex-col"}>
            <span className={"text-2xl border-b border-neutral-700 mb-4"}>General</span>
            <div>
                <div className={"max-w-2xl"}>
                    <label htmlFor="first_name"
                           className="block mb-1 text-base font-medium text-neutral-900 dark:text-white">Manufacturer</label>
                    <span
                        className="mb-4 bg-neutral-50 border border-neutral-300 text-neutral-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-neutral-700 dark:border-neutral-600 dark:placeholder-neutral-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    >{charger?.vendor}</span>
                </div>
                <div className={"max-w-2xl"}>
                    <label htmlFor="first_name"
                           className="block mb-1 text-base font-medium text-neutral-900 dark:text-white">Model</label>
                    <span
                        className="mb-4 bg-neutral-50 border border-neutral-300 text-neutral-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-neutral-700 dark:border-neutral-600 dark:placeholder-neutral-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    >{charger?.model}</span>
                </div>
                <div className={"max-w-2xl"}>
                    <label htmlFor="first_name"
                           className="block mb-1 text-base font-medium text-neutral-900 dark:text-white">Serial
                        Number</label>
                    <span
                        className="mb-4 bg-neutral-50 border border-neutral-300 text-neutral-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-neutral-700 dark:border-neutral-600 dark:placeholder-neutral-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    >{charger?.serialNumber}</span>
                </div>
            </div>
        </div>
        <div className={"mt-8 flex flex-col"}>
            <span className={"text-2xl border-b border-neutral-700 mb-4"}>Charger Actions</span>
            <div className={"flex flex-col max-w-2xl"}>
                <SoftRebootButton chargerId={chargerId}/>
                <HardRebootButton chargerId={chargerId}/>
            </div>
        </div>
    </div>
}