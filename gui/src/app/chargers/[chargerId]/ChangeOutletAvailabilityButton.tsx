"use client"

import {useCallback, useState} from "react";
import {changeEvseAvailability} from "@/app/chargers/[chargerId]/changeEvseAvailability";

export default function ChangeOutletAvailabilityButton({chargerId, outletId, status}: {
    chargerId: string,
    outletId: string,
    status: string | undefined
}) {
    const [loading, setLoading] = useState(false);
    const callback = useCallback(() => {
        setLoading(true)
        changeEvseAvailability(chargerId, outletId, status === "Unavailable").finally(() => setLoading(false))
    }, [setLoading, chargerId, outletId, status])

    const message = status === "Unavailable" ? "Enable outlet" : "Disable outlet"

    return <button type="button"
                   onClick={callback}
                   className="text-neutral-900 bg-white border border-neutral-300 focus:outline-none hover:bg-neutral-100 focus:ring-4 focus:ring-neutral-100 font-medium rounded-full text-sm px-5 py-2.5 me-2 mb-2 dark:bg-neutral-900 dark:text-white dark:border-neutral-600 dark:hover:bg-neutral-800 dark:hover:border-neutral-600 dark:focus:ring-neutral-700">{loading ? "Loading..." : message}
    </button>
}