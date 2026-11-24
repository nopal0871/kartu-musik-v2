export function createRecorder(stream, mimeType, videoBitsPerSecond) {
  return new MediaRecorder(stream, { mimeType, videoBitsPerSecond });
}
