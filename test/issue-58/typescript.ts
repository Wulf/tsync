/* This file is generated and managed by tsync */

export type CameraControl = Action & {
  camera_uuid: string;
}

export type Action =
  | Action__GetVideoParameterSettings;

type Action__GetVideoParameterSettings = {
  "action": "GetVideoParameterSettings";
  "json": VideoParameterSettings;
};

export interface VideoParameterSettings {
  frame_rate?: number;
}
