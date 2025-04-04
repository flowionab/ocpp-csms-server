// Code generated by protoc-gen-ts_proto. DO NOT EDIT.
// versions:
//   protoc-gen-ts_proto  v2.5.0
//   protoc               v5.29.0
// source: change_charger_availability.proto

/* eslint-disable */
import {BinaryReader, BinaryWriter} from "@bufbuild/protobuf/wire";

export const protobufPackage = "ocpp_csms_server";

export interface ChangeOutletAvailabilityRequest {
    chargerId: string;
    outletId: string;
    available: boolean;
}

export interface ChangeOutletAvailabilityResponse {
}

function createBaseChangeOutletAvailabilityRequest(): ChangeOutletAvailabilityRequest {
    return {chargerId: "", outletId: "", available: false};
}

export const ChangeOutletAvailabilityRequest: MessageFns<ChangeOutletAvailabilityRequest> = {
    encode(message: ChangeOutletAvailabilityRequest, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
        if (message.chargerId !== "") {
            writer.uint32(10).string(message.chargerId);
        }
        if (message.outletId !== "") {
            writer.uint32(18).string(message.outletId);
        }
        if (message.available !== false) {
            writer.uint32(24).bool(message.available);
        }
        return writer;
    },

    decode(input: BinaryReader | Uint8Array, length?: number): ChangeOutletAvailabilityRequest {
        const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = createBaseChangeOutletAvailabilityRequest();
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1: {
                    if (tag !== 10) {
                        break;
                    }

                    message.chargerId = reader.string();
                    continue;
                }
                case 2: {
                    if (tag !== 18) {
                        break;
                    }

                    message.outletId = reader.string();
                    continue;
                }
                case 3: {
                    if (tag !== 24) {
                        break;
                    }

                    message.available = reader.bool();
                    continue;
                }
            }
            if ((tag & 7) === 4 || tag === 0) {
                break;
            }
            reader.skip(tag & 7);
        }
        return message;
    },

    fromJSON(object: any): ChangeOutletAvailabilityRequest {
        return {
            chargerId: isSet(object.chargerId) ? globalThis.String(object.chargerId) : "",
            outletId: isSet(object.outletId) ? globalThis.String(object.outletId) : "",
            available: isSet(object.available) ? globalThis.Boolean(object.available) : false,
        };
    },

    toJSON(message: ChangeOutletAvailabilityRequest): unknown {
        const obj: any = {};
        if (message.chargerId !== "") {
            obj.chargerId = message.chargerId;
        }
        if (message.outletId !== "") {
            obj.outletId = message.outletId;
        }
        if (message.available !== false) {
            obj.available = message.available;
        }
        return obj;
    },

    create<I extends Exact<DeepPartial<ChangeOutletAvailabilityRequest>, I>>(base?: I): ChangeOutletAvailabilityRequest {
        return ChangeOutletAvailabilityRequest.fromPartial(base ?? ({} as any));
    },
    fromPartial<I extends Exact<DeepPartial<ChangeOutletAvailabilityRequest>, I>>(
        object: I,
    ): ChangeOutletAvailabilityRequest {
        const message = createBaseChangeOutletAvailabilityRequest();
        message.chargerId = object.chargerId ?? "";
        message.outletId = object.outletId ?? "";
        message.available = object.available ?? false;
        return message;
    },
};

function createBaseChangeOutletAvailabilityResponse(): ChangeOutletAvailabilityResponse {
    return {};
}

export const ChangeOutletAvailabilityResponse: MessageFns<ChangeOutletAvailabilityResponse> = {
    encode(_: ChangeOutletAvailabilityResponse, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
        return writer;
    },

    decode(input: BinaryReader | Uint8Array, length?: number): ChangeOutletAvailabilityResponse {
        const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = createBaseChangeOutletAvailabilityResponse();
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
            }
            if ((tag & 7) === 4 || tag === 0) {
                break;
            }
            reader.skip(tag & 7);
        }
        return message;
    },

    fromJSON(_: any): ChangeOutletAvailabilityResponse {
        return {};
    },

    toJSON(_: ChangeOutletAvailabilityResponse): unknown {
        const obj: any = {};
        return obj;
    },

    create<I extends Exact<DeepPartial<ChangeOutletAvailabilityResponse>, I>>(
        base?: I,
    ): ChangeOutletAvailabilityResponse {
        return ChangeOutletAvailabilityResponse.fromPartial(base ?? ({} as any));
    },
    fromPartial<I extends Exact<DeepPartial<ChangeOutletAvailabilityResponse>, I>>(
        _: I,
    ): ChangeOutletAvailabilityResponse {
        const message = createBaseChangeOutletAvailabilityResponse();
        return message;
    },
};

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

export type DeepPartial<T> = T extends Builtin ? T
    : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
        : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
            : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
                : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin ? P
    : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
    return value !== null && value !== undefined;
}

export interface MessageFns<T> {
    encode(message: T, writer?: BinaryWriter): BinaryWriter;

    decode(input: BinaryReader | Uint8Array, length?: number): T;

    fromJSON(object: any): T;

    toJSON(message: T): unknown;

    create<I extends Exact<DeepPartial<T>, I>>(base?: I): T;

    fromPartial<I extends Exact<DeepPartial<T>, I>>(object: I): T;
}
