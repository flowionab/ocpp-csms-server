import Link from "next/link";

export default async function Layout({children, params}: any) {
    const {chargerId} = await params;
    return <>
        <nav className={"h-12 border-gray-300 dark:bg-neutral-800 flex flex-row justify-center"}>
            <div className={"container flex flex-row"}>
                <Link href={`/chargers/${chargerId}`} className={"h-full flex flex-col pr-4"}>
                    <span className={"flex-1 content-center"}>Overview</span>
                </Link>
                <Link href={`/chargers/${chargerId}/configuration`} className={"h-full flex flex-col px-6"}>
                    <span className={"flex-1 content-center"}>Configuration</span>
                </Link>
                <Link href={`/chargers/${chargerId}/settings`} className={"h-full flex flex-col px-6"}>
                    <span className={"flex-1 content-center"}>Settings</span>
                </Link>
            </div>
        </nav>
        {children}
    </>
}