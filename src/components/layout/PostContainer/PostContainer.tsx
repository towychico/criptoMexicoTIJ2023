import react from "react"
import './postContainer.css'
import PostCard from "../PostCard/PostCard";

function PostContainer(){
    return(<div className="postContainer">
        <PostCard title="What is your favorite pizza flavor?" content="Mine is Margarita, a timeless classic, delights with tangy tomatoes, creamy mozzarella, & fresh basil on crispy crust. Nostalgic & irresistible, it's a symphony of flavors! A culinary masterpiece! 🍕😍 " type={1}/>
        <PostCard title="I dare you to Watch a Scary Movie This Halloween! 🍿👻" content="👀 Are you up for it? Gather your bravest friends, grab some popcorn, and dive into a world of supernatural suspense. Share your movie picks and reactions" type={0}/>
        <PostCard title="Post 1" content="what is the best movie for halloween?" type={1}/>
        <PostCard title="Post 1" content="what is the best movie for halloween?" type={0}/>
        <PostCard title="Post 1" content="what is the best movie for halloween?" type={1}/>

    </div>)
}
// eslint-disable-next-line import/no-default-export
export default PostContainer;