"use client"

import {hardRebootCharger} from "@/app/chargers/[chargerId]/settings/hardRebootCharger";
import {useCallback, useState} from "react";

export default function HardRebootButton({chargerId}: { chargerId: string }) {
    const [loading, setLoading] = useState(false);
    const callback = useCallback(() => {
        setLoading(true)
        hardRebootCharger(chargerId).finally(() => setLoading(false))
    }, [setLoading, chargerId])

    return <button type="button"
                   onClick={callback}
                   className="text-neutral-900 bg-white border border-neutral-300 focus:outline-none hover:bg-neutral-100 focus:ring-4 focus:ring-neutral-100 font-medium rounded-full text-sm px-5 py-2.5 me-2 mb-2 dark:bg-neutral-900 dark:text-white dark:border-neutral-600 dark:hover:bg-neutral-800 dark:hover:border-neutral-600 dark:focus:ring-neutral-700">{loading ? "Loading..." : "Hard Reboot"}
    </button>
}