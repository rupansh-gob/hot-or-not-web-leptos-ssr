mod ic;
mod posts;
mod speculation;

use candid::Principal;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

use crate::{component::spinner::FullScreenSpinner, state::canisters::unauth_canisters};
use ic::ProfileDetails;

use posts::ProfilePosts;
use speculation::ProfileSpeculations;

#[derive(Params, PartialEq)]
struct ProfileParams {
    id: String,
}

#[component]
fn Stat(stat: u64, #[prop(into)] info: String) -> impl IntoView {
    view! {
        <div class="flex flex-1 flex-col items-center text-white space-y-0.5">
            <span class="font-bold text-xl">{stat}</span>
            <span class="text-md">{info}</span>
        </div>
    }
}

#[component]
fn ListSwitcher(user_canister: Principal) -> impl IntoView {
    let (cur_tab, set_cur_tab) = create_query_signal::<String>("tab");
    let current_tab = create_memo(move |_| {
        with!(|cur_tab| match cur_tab.as_deref() {
            Some("posts") => 0,
            Some("speculations") => 1,
            _ => 0,
        })
    });
    let tab_class = move |tab_id: usize| {
        if tab_id == current_tab() {
            "text-orange-500 border-b-4 border-orange-500 flex justify-center w-full py-2"
        } else {
            "text-white flex justify-center w-full py-2"
        }
    };

    view! {
        <div class="relative flex flex-row w-7/12 text-center text-md md:text-lg lg:text-xl xl:text-2xl">
            <button class=move || tab_class(0) on:click=move |_| set_cur_tab(Some("posts".into()))>
                <Icon icon=icondata::FiGrid/>
            </button>
            <button
                class=move || tab_class(1)
                on:click=move |_| set_cur_tab(Some("speculations".into()))
            >
                <Icon icon=icondata::BsTrophy/>
            </button>
        </div>
        <div class="flex flex-col gap-y-12 justify-center w-7/12">
            <Show
                when=move || current_tab() == 0
                fallback=move || view! { <ProfileSpeculations user_canister/> }
            >
                <ProfilePosts user_canister/>
            </Show>
        </div>
    }
}

#[component]
fn ProfileViewInner(user: ProfileDetails, user_canister: Principal) -> impl IntoView {
    let username_or_principal = user.username_or_principal();
    let profile_pic = user.profile_pic_or_random();
    let display_name = user.display_name_or_fallback();
    let earnings = user.lifetime_earnings;

    view! {
        <div class="min-h-screen bg-black overflow-y-scroll py-10">
            <div class="grid grid-cols-1 gap-5 justify-normal justify-items-center w-full">
                <img
                    class="h-24 w-24 rounded-full"
                    alt=username_or_principal.clone()
                    src=profile_pic
                />
                <div class="flex flex-col text-center">
                    <span class="text-md text-white font-bold">{display_name}</span>
                    <div class="text-sm flex flex-row">
                        <p class="text-white">@ {username_or_principal}</p>
                        <p class="text-white text-md font-bold px-1">{" • "}</p>
                        <p class="text-orange-500">{earnings} Earnings</p>
                    </div>
                </div>
                <div class="flex justify-around text-center rounded-full divide-x-2 divide-white/20 bg-white/10 p-4 my-4 w-7/12">
                    <Stat stat=user.followers_cnt info="Lovers"/>
                    <Stat stat=user.following_cnt info="Loving"/>
                    <Stat stat=user.hots info="Hots"/>
                    <Stat stat=user.nots info="Nots"/>
                </div>
                <ListSwitcher user_canister/>
            </div>
        </div>
    }
}

#[component]
pub fn ProfileView() -> impl IntoView {
    let params = use_params::<ProfileParams>();
    let principal_or_username = move || {
        params.with(|p| {
            let ProfileParams { id } = p.as_ref().ok()?;

            let res = Principal::from_text(id).map_err(|_| id.clone());
            Some(res)
        })
    };

    let user_details = create_resource(principal_or_username, |principal_or_username| async move {
        let canisters = unauth_canisters();
        let user_index = canisters.user_index();
        let user_canister = match principal_or_username? {
            Ok(p) => user_index
                .get_user_canister_id_from_user_principal_id(p)
                .await
                .ok()??,
            Err(u) => user_index
                .get_user_canister_id_from_unique_user_name(u)
                .await
                .ok()??,
        };
        let user = canisters.individual_user(user_canister);
        let user_details = user.get_profile_details().await.ok()?;
        Some((user_details.into(), user_canister))
    });

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                user_details
                    .get()
                    .map(|user| {
                        view! {
                            {move || {
                                if let Some((user, user_canister)) = user.clone() {
                                    view! { <ProfileViewInner user user_canister/> }
                                } else {
                                    view! { <Redirect path="/"/> }
                                }
                            }}
                        }
                    })
            }}

        </Suspense>
    }
}
