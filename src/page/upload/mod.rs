mod cf_upload;
mod validators;
mod video_upload;

use crate::component::toggle::Toggle;

use leptos::{
    html::{Input, Textarea},
    *,
};

use validators::{description_validator, hashtags_validator};
use video_upload::{FileWithUrl, PreVideoUpload, VideoUploader};

#[derive(Clone)]
struct UploadParams {
    file_blob: FileWithUrl,
    hashtags: Vec<String>,
    description: String,
    enable_hot_or_not: bool,
    is_nsfw: bool,
}

#[component]
fn PreUploadView(trigger_upload: WriteSignal<Option<UploadParams>>) -> impl IntoView {
    let description_err = create_rw_signal(String::new());
    let desc_err_memo = create_memo(move |_| description_err());
    let hashtags = create_rw_signal(Vec::new());
    let hashtags_err = create_rw_signal(String::new());
    let hashtags_err_memo = create_memo(move |_| hashtags_err());
    let file_blob = create_rw_signal(None::<FileWithUrl>);
    let invalid_form = create_memo(move |_| {
        with!(|desc_err_memo, hashtags_err_memo, file_blob| {
            !desc_err_memo.is_empty() || !hashtags_err_memo.is_empty() || file_blob.is_none()
        })
    });
    let desc = create_node_ref::<Textarea>();
    let hashtag_inp = create_node_ref::<Input>();
    let enable_hot_or_not = create_node_ref::<Input>();
    let is_nsfw = create_node_ref::<Input>();
    let on_submit = move || {
        let description = desc.get_untracked().unwrap().value();
        let hashtags = hashtags.get_untracked();
        let Some(file_blob) = file_blob.get_untracked() else {
            return;
        };
        trigger_upload.set(Some(UploadParams {
            file_blob,
            hashtags,
            description,
            enable_hot_or_not: enable_hot_or_not
                .get_untracked()
                .map(|v| v.checked())
                .unwrap_or_default(),
            is_nsfw: is_nsfw
                .get_untracked()
                .map(|v| v.checked())
                .unwrap_or_default(),
        }));
    };

    let hashtag_on_input = move |hts| match hashtags_validator(hts) {
        Ok(hts) => {
            hashtags.set(hts);
            hashtags_err.set(String::new());
        }
        Err(e) => hashtags_err.set(e),
    };

    create_effect(move |_| {
        let Some(hashtag_inp) = hashtag_inp() else {
            return;
        };

        let val = hashtag_inp.value();
        if !val.is_empty() {
            hashtag_on_input(val);
        }
    });

    view! {
        <PreVideoUpload file_blob=file_blob.write_only()/>
        <div class="flex flex-col basis-full lg:basis-7/12 gap-4">
            <div class="flex flex-col gap-y-2">
                <Show when=move || { with!(| description_err | ! description_err.is_empty()) }>
                    <span class="text-red-500 text-sm">{desc_err_memo()}</span>
                </Show>
                <textarea
                    _ref=desc
                    on:input=move |ev| {
                        let desc = event_target_value(&ev);
                        description_err.set(description_validator(desc).err().unwrap_or_default());
                    }

                    class="p-4 bg-neutral-800 rounded-md min-w-full"
                    rows=3
                    placeholder="Write your description here.."
                ></textarea>
            </div>
            <div class="flex flex-col gap-y-2">
                <Show
                    when=move || { with!(| hashtags_err | ! hashtags_err.is_empty()) }
                    fallback=|| {
                        view! { <h3 class="font-semibold text-neutral-600">Add Hashtags</h3> }
                    }
                >

                    <h3 class="text-red-500 font-semibold">{hashtags_err_memo()}</h3>
                </Show>
                <input
                    _ref=hashtag_inp
                    on:input=move |ev| {
                        let hts = event_target_value(&ev);
                        hashtag_on_input(hts);
                    }

                    class="p-4 py-5 bg-neutral-800 rounded-md"
                    type="text"
                    placeholder="#hashtag1,#hashtag2,#hashtag3..."
                />
            </div>
            <div class="flex flex-col gap-y-2">
                <Toggle node_ref=enable_hot_or_not lab="Participate in Hot or Not"/>
                <Toggle lab="NSFW"/>
            </div>
            <button
                on:click=move |_| on_submit()
                disabled=invalid_form
                class="py-3 w-5/6 md:w-4/6 my-8 self-center disabled:bg-orange-400 disabled:text-white/80 bg-orange-600 rounded-full font-bold text-md md:text-lg lg:text-xl"
            >
                Upload Video
            </button>
        </div>
    }
}

#[component]
pub fn UploadPostPage() -> impl IntoView {
    let trigger_upload = create_rw_signal(None::<UploadParams>);

    view! {
        <div class="flex flex-col h-screen items-center overflow-y-scroll gap-6 md:gap-8 lg:gap-16 w-full py-4 md:py-6 px-3 md:px-6 lg:px-10 bg-black text-white">
            <h1 class="font-bold text-lg md:text-xl text-center">Upload</h1>
            <div class="flex flex-row flex-wrap h-full w-full">
                <Show
                    when=move || { with!(| trigger_upload | trigger_upload.is_some()) }
                    fallback=move || {
                        view! { <PreUploadView trigger_upload=trigger_upload.write_only()/> }
                    }
                >

                    <VideoUploader params=trigger_upload.get_untracked().unwrap()/>
                </Show>
            </div>
        </div>
    }
}
