"use client"


import {softRebootCharger} from "@/app/chargers/[chargerId]/settings/softRebootCharger";
import {useCallback, useState} from "react";

export default function SoftRebootButton({chargerId}: { chargerId: string }) {
    const [loading, setLoading] = useState(false);
    const callback = useCallback(() => {
        setLoading(true)
        softRebootCharger(chargerId).finally(() => setLoading(false))
    }, [setLoading, chargerId])

    return <button type="button"
                   disabled={loading}
                   onClick={callback}
                   className="text-white bg-neutral-800 hover:bg-neutral-900 focus:outline-none focus:ring-4 focus:ring-neutral-300 font-medium rounded-full text-sm px-5 py-2.5 me-2 mb-2 dark:bg-neutral-800 dark:hover:bg-neutral-700 dark:focus:ring-neutral-700 dark:border-neutral-700">{loading ? "loading..." : "Reboot Charger"}
    </button>
}