#[cfg_attr(target_os = "windows", path = "media_service/windows.rs")]
#[cfg_attr(not(target_os = "windows"), path = "media_service/unknown.rs")]
mod media_service;

use media_service::MediaService;
use neon::prelude::*;
use std::cell::RefCell;

type BoxedMediaService = JsBox<RefCell<MediaService>>;

fn create_media_service(mut cx: FunctionContext) -> JsResult<BoxedMediaService> {
    let service = RefCell::new(MediaService::new());
    Ok(cx.boxed(service))
}

// region Control
fn media_service_is_enabled(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let is_enabled = service.is_enabled();

    Ok(cx.boolean(is_enabled))
}

fn media_service_set_is_enabled(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let enabled = cx.argument::<JsBoolean>(1)?.value(&mut cx);
    service.set_is_enabled(enabled);

    Ok(cx.undefined())
}
// endregion Control

// region Buttons
fn media_service_is_play_enabled(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let is_play_enabled = service.is_play_enabled();

    Ok(cx.boolean(is_play_enabled))
}

fn media_service_set_is_play_enabled(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let enabled = cx.argument::<JsBoolean>(1)?.value(&mut cx);
    service.set_is_play_enabled(enabled);

    Ok(cx.undefined())
}

fn media_service_is_pause_enabled(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let is_play_enabled = service.is_pause_enabled();

    Ok(cx.boolean(is_play_enabled))
}

fn media_service_set_is_pause_enabled(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let enabled = cx.argument::<JsBoolean>(1)?.value(&mut cx);
    service.set_is_pause_enabled(enabled);

    Ok(cx.undefined())
}

fn media_service_is_previous_enabled(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let is_play_enabled = service.is_previous_enabled();

    Ok(cx.boolean(is_play_enabled))
}

fn media_service_set_is_previous_enabled(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let enabled = cx.argument::<JsBoolean>(1)?.value(&mut cx);
    service.set_is_previous_enabled(enabled);

    Ok(cx.undefined())
}

fn media_service_is_next_enabled(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let is_play_enabled = service.is_next_enabled();

    Ok(cx.boolean(is_play_enabled))
}

fn media_service_set_is_next_enabled(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let enabled = cx.argument::<JsBoolean>(1)?.value(&mut cx);
    service.set_is_next_enabled(enabled);

    Ok(cx.undefined())
}
// endregion Buttons

// region Media Information
fn media_service_get_media_type(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let media_type = service.get_media_type();

    Ok(cx.number(media_type))
}

fn media_service_set_media_type(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let media_type = cx.argument::<JsNumber>(1)?.value(&mut cx);
    service.set_media_type(media_type as i32);

    Ok(cx.undefined())
}

fn media_service_get_playback_status(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let playback_status = service.get_playback_status();

    Ok(cx.number(playback_status))
}

fn media_service_set_playback_status(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let playback_status = cx.argument::<JsNumber>(1)?.value(&mut cx);
    service.set_playback_status(playback_status as i32);

    Ok(cx.undefined())
}

fn media_service_get_artist(mut cx: FunctionContext) -> JsResult<JsString> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let artist = service.get_artist();

    Ok(cx.string(artist))
}

fn media_service_set_artist(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let artist = cx.argument::<JsString>(1)?.value(&mut cx);
    service.set_artist(artist);

    Ok(cx.undefined())
}

fn media_service_get_album_artist(mut cx: FunctionContext) -> JsResult<JsString> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let album_artist = service.get_album_artist();

    Ok(cx.string(album_artist))
}

fn media_service_set_album_artist(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let album_artist = cx.argument::<JsString>(1)?.value(&mut cx);
    service.set_album_artist(album_artist);

    Ok(cx.undefined())
}

fn media_service_get_album_title(mut cx: FunctionContext) -> JsResult<JsString> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let album_title = service.get_album_title();

    Ok(cx.string(album_title))
}

fn media_service_set_album_title(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let album_title = cx.argument::<JsString>(1)?.value(&mut cx);
    service.set_album_title(album_title);

    Ok(cx.undefined())
}

fn media_service_get_title(mut cx: FunctionContext) -> JsResult<JsString> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow();

    let title = service.get_title();

    Ok(cx.string(title))
}

fn media_service_set_title(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let title = cx.argument::<JsString>(1)?.value(&mut cx);
    service.set_title(title);

    Ok(cx.undefined())
}

fn media_service_set_thumbnail(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let service = cx.argument::<BoxedMediaService>(0)?;
    let service = service.borrow_mut();

    let thumbnail_type = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let thumbnail = cx.argument::<JsString>(2)?.value(&mut cx);
    service.set_thumbnail(thumbnail_type as i32, thumbnail);

    Ok(cx.undefined())
}
// endregion Media Information

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("createMediaService", create_media_service)?;
    // region Control
    cx.export_function("mediaServiceIsEnabled", media_service_is_enabled)?;
    cx.export_function("mediaServiceSetIsEnabled", media_service_set_is_enabled)?;
    // endregion Control
    // region Buttons
    cx.export_function("mediaServiceIsPlayEnabled", media_service_is_play_enabled)?;
    cx.export_function(
        "mediaServiceSetIsPlayEnabled",
        media_service_set_is_play_enabled,
    )?;
    cx.export_function("mediaServiceIsPauseEnabled", media_service_is_pause_enabled)?;
    cx.export_function(
        "mediaServiceSetIsPauseEnabled",
        media_service_set_is_pause_enabled,
    )?;
    cx.export_function(
        "mediaServiceIsPreviousEnabled",
        media_service_is_previous_enabled,
    )?;
    cx.export_function(
        "mediaServiceSetIsPreviousEnabled",
        media_service_set_is_previous_enabled,
    )?;
    cx.export_function("mediaServiceIsNextEnabled", media_service_is_next_enabled)?;
    cx.export_function(
        "mediaServiceSetIsNextEnabled",
        media_service_set_is_next_enabled,
    )?;
    // endregion Buttons
    // region Media Information
    cx.export_function("mediaServiceGetMediaType", media_service_get_media_type)?;
    cx.export_function("mediaServiceSetMediaType", media_service_set_media_type)?;
    cx.export_function(
        "mediaServiceGetPlaybackStatus",
        media_service_get_playback_status,
    )?;
    cx.export_function(
        "mediaServiceSetPlaybackStatus",
        media_service_set_playback_status,
    )?;
    cx.export_function("mediaServiceGetArtist", media_service_get_artist)?;
    cx.export_function("mediaServiceSetArtist", media_service_set_artist)?;
    cx.export_function("mediaServiceGetAlbumArtist", media_service_get_album_artist)?;
    cx.export_function("mediaServiceSetAlbumArtist", media_service_set_album_artist)?;
    cx.export_function("mediaServiceGetAlbumTitle", media_service_get_album_title)?;
    cx.export_function("mediaServiceSetAlbumTitle", media_service_set_album_title)?;
    cx.export_function("mediaServiceGetTitle", media_service_get_title)?;
    cx.export_function("mediaServiceSetTitle", media_service_set_title)?;
    cx.export_function("mediaServiceSetThumbnail", media_service_set_thumbnail)?;
    // endregion Media Information
    Ok(())
}
