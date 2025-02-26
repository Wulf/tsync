/* This file is generated and managed by tsync */

type CameraControl = Action & {
  camera_uuid: string;
}

type Action =
  | Action__GetVideoParameterSettings;

type Action__GetVideoParameterSettings = {
  "action": "GetVideoParameterSettings";
  "json": VideoParameterSettings;
};

interface VideoParameterSettings {
  frame_rate?: number;
}
