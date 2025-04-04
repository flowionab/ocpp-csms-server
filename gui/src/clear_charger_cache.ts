// Code generated by protoc-gen-ts_proto. DO NOT EDIT.
// versions:
//   protoc-gen-ts_proto  v2.5.0
//   protoc               v5.29.0
// source: clear_charger_cache.proto

/* eslint-disable */
import { BinaryReader, BinaryWriter } from "@bufbuild/protobuf/wire";

export const protobufPackage = "ocpp_csms_server";

export interface ClearChargerCacheRequest {
  chargerId: string;
}

export interface ClearChargerCacheResponse {
}

function createBaseClearChargerCacheRequest(): ClearChargerCacheRequest {
  return { chargerId: "" };
}

export const ClearChargerCacheRequest: MessageFns<ClearChargerCacheRequest> = {
  encode(message: ClearChargerCacheRequest, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
    if (message.chargerId !== "") {
      writer.uint32(10).string(message.chargerId);
    }
    return writer;
  },

  decode(input: BinaryReader | Uint8Array, length?: number): ClearChargerCacheRequest {
    const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseClearChargerCacheRequest();
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
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skip(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): ClearChargerCacheRequest {
    return { chargerId: isSet(object.chargerId) ? globalThis.String(object.chargerId) : "" };
  },

  toJSON(message: ClearChargerCacheRequest): unknown {
    const obj: any = {};
    if (message.chargerId !== "") {
      obj.chargerId = message.chargerId;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<ClearChargerCacheRequest>, I>>(base?: I): ClearChargerCacheRequest {
    return ClearChargerCacheRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ClearChargerCacheRequest>, I>>(object: I): ClearChargerCacheRequest {
    const message = createBaseClearChargerCacheRequest();
    message.chargerId = object.chargerId ?? "";
    return message;
  },
};

function createBaseClearChargerCacheResponse(): ClearChargerCacheResponse {
  return {};
}

export const ClearChargerCacheResponse: MessageFns<ClearChargerCacheResponse> = {
  encode(_: ClearChargerCacheResponse, writer: BinaryWriter = new BinaryWriter()): BinaryWriter {
    return writer;
  },

  decode(input: BinaryReader | Uint8Array, length?: number): ClearChargerCacheResponse {
    const reader = input instanceof BinaryReader ? input : new BinaryReader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseClearChargerCacheResponse();
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

  fromJSON(_: any): ClearChargerCacheResponse {
    return {};
  },

  toJSON(_: ClearChargerCacheResponse): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<ClearChargerCacheResponse>, I>>(base?: I): ClearChargerCacheResponse {
    return ClearChargerCacheResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<ClearChargerCacheResponse>, I>>(_: I): ClearChargerCacheResponse {
    const message = createBaseClearChargerCacheResponse();
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
